use ::anyhow::Result;
use ::fastwebsockets::{FragmentCollector, Frame, Payload, OpCode};
use ::fastwebsockets::upgrade::UpgradeFut;
use ::hyper::upgrade::Upgraded;
use ::hyper_util::rt::TokioIo;
use ::log::{info, error};
use ::tokio_util::sync::CancellationToken;

use super::enums::Commands;
use super::payload::Command;
use super::queue::getMessageQueue;

#[derive()]
pub struct WebSocketClient
{
	id: i64,
	socket: FragmentCollector<TokioIo<Upgraded>>,
}

unsafe impl Send for WebSocketClient {}

impl WebSocketClient
{
	pub async fn fromUpgradeFut(future: UpgradeFut) -> Result<Self>
	{
		let ws = future.await?;
		return Ok(Self
		{
			id: -1,
			socket: FragmentCollector::new(ws),
		})
	}
	
	pub async fn start(&mut self, token: CancellationToken) -> Result<()>
	{
		if let Ok(queue) = getMessageQueue().lock()
		{
			println!("Client id: {}", self.id);
			queue.queueBroadcast(format!("Client {} is connected!", self.id))?;
			queue.queueCommand(self.id, Commands::AuthenticateRequest, None)?;
		}
		
		//Send auth request before waiting for input
		self.sendQueuedMessages().await?;
		
		loop
		{
			tokio::select! {
				_ = token.cancelled() => {
					info!("Graceful shutdown - Client loop ending");
					break;
				},
				
				result = self.socket.read_frame() => {
					if let Ok(frame) = result
					{
						if self.poll(frame).await?
						{
							break;
						}
					}
				}
			}
		}
		
		return Ok(());
	}
	
	async fn poll(&mut self, frame: Frame<'_>) -> Result<bool>
	{
		match frame.opcode
		{
			OpCode::Close => {
				if let Ok(messager) = getMessageQueue().lock()
				{
					let _ = messager.removeId(self.id);
				}
				info!("Client disconnected!");
				return Ok(true);
			},
			
			OpCode::Text => self.processText(frame).await?,
			
			/*
			OpCode::Binary => {
				self.socket.write_frame(frame).await?;
			},
			*/
			
			_ => {},
		}
		
		self.sendQueuedMessages().await?;
		
		return Ok(false);
	}
	
	async fn processText(&mut self, frame: Frame<'_>) -> Result<()>
	{
		match std::str::from_utf8(frame.payload.as_ref())
		{
			Ok(json) => {
				let command: Command = serde_json::from_str(json)?;
				match command.Type
				{
					Commands::BroadcastSend => {
						if let Ok(queue) = getMessageQueue().try_lock()
						{
							queue.queueBroadcast(command.Data["text"].to_owned())?;
						}
					},
					
					Commands::AuthenticateSend => {
						println!("Authentication data received!");
						if let Ok(queue) = getMessageQueue().try_lock()
						{
							queue.queueCommand(self.id, Commands::AuthenticateSuccess, None)?;
						}
						/*
						//TODO: Expand this to safely handle ID collisions; Probably will require actual authentication instead of the client just declaring an id
							//Update the id in the message queue as well
							if let Ok(queue) = getMessageQueue().try_lock()
							{
								queue.removeId(self.id);
								self.id = obj.Id;
								queue.registerId(self.id);
							}
							
							info!("Client id {}", self.id);
							
							let response = Frame::text(Payload::Owned(format!("Client ID set as {}", self.id).into_bytes()));
							self.socket.write_frame(response).await?;
						*/
					},
					
					_ => {},
				}
			},
			
			Err(e) => error!("Failed to parse text packet: {:?}", e),
		}
		
		return Ok(());
	}
	
	async fn sendQueuedMessages(&mut self) -> Result<()>
	{
		let messages = match getMessageQueue().try_lock()
		{
			Ok(queue) => queue.readMessages(self.id),
			Err(_) => vec![],
		};
		
		let json = serde_json::to_string(&messages)?;
		self.socket.write_frame(
			Frame::text(
				Payload::Owned(json.into_bytes())
			)
		).await?;
		
		return Ok(());
	}
}
