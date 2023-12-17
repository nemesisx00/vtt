use ::anyhow::Result;
use ::fastwebsockets::{FragmentCollector, Frame, Payload, OpCode};
use ::fastwebsockets::upgrade::UpgradeFut;
use ::hyper::upgrade::Upgraded;
use ::hyper_util::rt::TokioIo;
use ::log::{info, error};
use ::tokio_util::sync::CancellationToken;
use super::payload::{Broadcast, ClientIdentity};
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
			queue.queueMessage(self.id, format!("Client {} is connected!", self.id));
		}
		
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
			Ok(s) => {
				if let Some((typeName, json)) = s.split_once("{")
				{
					let json = format!("{{{}", json);
					match typeName
					{
						Broadcast::Name => {
							let obj: Broadcast = serde_json::from_str(json.as_str())?;
							if let Ok(queue) = getMessageQueue().try_lock()
							{
								queue.queueBroadcast(obj.text.to_owned());
							}
						},
						
						ClientIdentity::Name => {
							let obj: ClientIdentity = serde_json::from_str(json.as_str())?;
							
							//TODO: Expand this to safely handle ID collisions; Probably will require actual authentication instead of the client just declaring an id
							//Update the id in the message queue as well
							if let Ok(queue) = getMessageQueue().try_lock()
							{
								queue.removeId(self.id);
								self.id = obj.id;
								queue.registerId(self.id);
							}
							
							info!("Client id {}", self.id);
							
							let response = Frame::text(Payload::Owned(format!("Client ID set as {}", self.id).into_bytes()));
							self.socket.write_frame(response).await?;
						},
						
						_ => {},
					};
				}
			},
			
			Err(e) => error!("Failed to parse text packet: {:?}", e),
		}
		
		return Ok(());
	}
	
	async fn sendQueuedMessages(&mut self) -> Result<()>
	{
		let mut messages = vec![];
		if let Ok(queue) = getMessageQueue().try_lock()
		{
			messages = queue.readMessages(self.id);
		}
		
		for message in messages
		{
			let frame = Frame::text(Payload::Owned(message.into_bytes()));
			self.socket.write_frame(frame).await?;
		}
		
		return Ok(());
	}
}
