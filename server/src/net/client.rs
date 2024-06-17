use std::collections::HashMap;
use ::anyhow::Result;
use chrono::{Duration, Utc};
use ::fastwebsockets::{FragmentCollector, Frame, Payload, OpCode};
use ::fastwebsockets::upgrade::UpgradeFut;
use ::hyper::upgrade::Upgraded;
use ::hyper_util::rt::TokioIo;
use ::log::{info, error};
use ::tokio_util::sync::CancellationToken;
use crate::data::dao;
use crate::data::{Message, User};
use crate::net::user::getUserManager;
use super::enums::Commands;
use super::payload::Command;
use super::queue::getMessageQueue;

pub struct WebSocketClient
{
	id: i64,
	user: Option<User>,
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
			user: None,
			socket: FragmentCollector::new(ws),
		});
	}
	
	pub async fn start(&mut self, token: CancellationToken) -> Result<()>
	{
		self.queueCommand(self.id, Commands::AuthenticateRequest, None)?;
		//Send auth request before waiting for input
		self.sendQueuedMessages().await?;
		
		loop
		{
			tokio::select! {
				_ = token.cancelled() => {
					info!("Graceful shutdown - Client {} loop ending", self.id);
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
	
	fn username(&self) -> String
	{
		return match &self.user
		{
			Some(u) => u.name.to_owned(),
			None => self.id.to_string(),
		};
	}
	
	async fn poll(&mut self, frame: Frame<'_>) -> Result<bool>
	{
		match frame.opcode
		{
			OpCode::Close => {
				self.queueRemoveId();
				
				let name = match &self.user
				{
					None => String::default(),
					Some(u) => u.name.to_owned(),
				};
				
				info!("{} ({}) disconnected!", name, self.id);
				self.queueBroadcast(format!("{} ({}) disconnected!", name, self.id))?;
				
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
					Commands::BroadcastSend => self.handleBroadcastSend(command).await?,
					Commands::AuthenticateSend => self.handleAuthenticateSend(command).await?,
					_ => {},
				}
			},
			
			Err(e) => error!("Failed to parse text packet: {:?}", e),
		}
		
		return Ok(());
	}
	
	// -----
	
	async fn handleAuthenticateSend(&mut self, command: Command) -> Result<()>
	{
		println!("Authentication data received!");
		
		match command.Data.get("name")
		{
			None => self.queueCommand(self.id, Commands::AuthenticateFail, None)?,
			
			Some(username) => {
				{
					self.user = match dao::userFind(username.clone()).await
					{
						Err(e) => {
							error!("Error searching the db for username '{}': {:?}", username, e);
							None
						},
						Ok(opt) => match opt
						{
							None => {
								let content = User
								{
									name: username.to_owned(),
									..Default::default()
								};
								dao::userCreate(Some(content)).await?
							},
							Some(u) => Some(u),
						},
					};
				}
				
				match &self.user
				{
					Some(user) => {
						match &user.id
						{
							Some(id) => {
								if let Some(newId) = self.userGetClientId(&id.to_string())
								{
									self.id = newId;
									
									let start = (Utc::now() - Duration::days(1)).naive_utc();
									let end = Utc::now().naive_utc();
									let messages = dao::messageFindByDateRange(start, end).await?;
									
									for m in messages
									{
										let data: HashMap<String, String> = vec![
											("text".to_string(), m.text.to_owned()),
										].into_iter().collect();
										self.queueCommand(self.id.to_owned(), Commands::BroadcastReceive, Some(data))?;
									}
									
									let data: HashMap<String, String> = vec![
										("clientId".to_string(), self.id.to_string()),
										("username".to_string(), user.name.to_owned()),
									].into_iter().collect();
									
									self.queueCommand(self.id, Commands::AuthenticateSuccess, Some(data))?;
									self.queueBroadcast(format!("{} ({}) connected!", user.name, self.id))?;
								}
							},
							None => self.queueCommand(self.id, Commands::AuthenticateFail, None)?,
						}
					},
					None => self.queueCommand(self.id, Commands::AuthenticateFail, None)?,
				}
			}
		}
		
		return Ok(());
	}
	
	async fn handleBroadcastSend(&mut self, command: Command) -> Result<()>
	{
		if let Some(text) = command.Data.get("text")
		{
			//TODO: Implement input sanitation
			if !text.is_empty()
			{
				if let Some(user) = &self.user
				{
					self.queueBroadcast(format!("{}: {}", self.username(), text))?;
					
					let content = Message
					{
						text: text.to_owned(),
						timestamp: Utc::now().timestamp(),
						userId: user.id.clone(),
						..Default::default()
					};
					
					_ = dao::messageCreate(Some(content)).await?;
				}
			}
		}
		
		return Ok(());
	}
	
	// -----
	
	fn queueBroadcast(&self, text: String) -> Result<()>
	{
		if let Ok(queue) = getMessageQueue().lock()
		{
			queue.queueBroadcast(text)?;
		}
		
		return Ok(());
	}
	
	fn queueCommand(&self, id: i64, command: Commands, data: Option<HashMap<String, String>>) -> Result<()>
	{
		if let Ok(queue) = getMessageQueue().lock()
		{
			queue.queueCommand(id, command, data)?;
		}
		
		return Ok(());
	}
	
	fn queueRemoveId(&self)
	{
		if let Ok(messager) = getMessageQueue().lock()
		{
			let _ = messager.removeId(self.id);
		}
	}
	
	async fn sendQueuedMessages(&mut self) -> Result<()>
	{
		let messages = match getMessageQueue().lock()
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
	
	fn userGetClientId(&self, userId: &String) -> Option<i64>
	{
		let mut clientId = None;
		if let Ok(userManager) = getUserManager().lock()
		{
			clientId = userManager.getClientId(userId);
		}
		
		return clientId;
	}
}
