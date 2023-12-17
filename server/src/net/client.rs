use ::anyhow::Result;
use fastwebsockets::{Frame, Payload};
use ::fastwebsockets::{FragmentCollector, OpCode};
use ::fastwebsockets::upgrade::UpgradeFut;
use ::hyper::upgrade::Upgraded;
use ::hyper_util::rt::TokioIo;
use ::log::{info, error};
use tokio_util::sync::CancellationToken;
use super::messager::getMessageQueue;

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
					info!("Client loop ending");
					break;
				},
				
				result = self.socket.read_frame() => {
					if let Ok(frame) = result
					{
						match frame.opcode
						{
							OpCode::Close => {
								if let Ok(messager) = getMessageQueue().lock()
								{
									let _ = messager.removeId(self.id);
								}
								info!("Client disconnected!");
								break;
							},
							
							//Echo back
							OpCode::Text | OpCode::Binary => {
								if frame.opcode == OpCode::Text
								{
									match std::str::from_utf8(frame.payload.as_ref())
									{
										Ok(s) => info!("Input: '{}'", s),
										Err(e) => error!("Failed to parse packet: {:?}", e),
									}
								}
								
								self.socket.write_frame(frame).await?;
							},
							
							_ => {},
						}
						
						let mut messages = vec![];
						if let Ok(queue) = getMessageQueue().lock()
						{
							messages = queue.readMessages(self.id);
						}
						
						for message in messages
						{
							let frame = Frame::text(Payload::Owned(message.into_bytes()));
							self.socket.write_frame(frame).await?;
						}
					}
				}
			}
		}
		
		return Ok(());
	}
}
