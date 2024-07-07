use std::collections::HashMap;
use ::anyhow::Result;
use ::base64::prelude::*;
use ::chrono::{NaiveDateTime, Utc};
use ::fastwebsockets::{FragmentCollector, Frame, Payload, OpCode};
use ::fastwebsockets::upgrade::UpgradeFut;
use ::hyper::upgrade::Upgraded;
use ::hyper_util::rt::TokioIo;
use ::log::{info, error};
use ::tokio_util::sync::CancellationToken;
use crate::data::dao;
use crate::data::{Message, User};
use crate::data::assets::{loadAsset, Asset, Image};
use crate::net::user::getUserManager;
use crate::util::parseDateTime;
use super::commands::Commands;
use super::payload::Command;
use super::queue::getMessageQueue;

pub struct WebSocketClient
{
	id: i64,
	user: Option<User>,
	socket: FragmentCollector<TokioIo<Upgraded>>,
}

unsafe impl Send for WebSocketClient {}
unsafe impl Sync for WebSocketClient {}

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
		self.queueCommandSimple(self.id, Commands::AuthenticateRequest)?;
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
					Commands::AuthenticateSend => self.handleAuthenticateSend(command).await?,
					Commands::BroadcastGetRequest => self.handleBroadcastGetRequest(command).await?,
					Commands::BroadcastRequest => self.handleBroadcastSend(command).await?,
					Commands::Scene2DRequest => self.handleScene2dRequest(command).await?,
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
			None => self.queueCommand(self.id, Commands::AuthenticateFail, None, None)?,
			
			Some(username) => {
				self.user = self.userFindOrCreate(username).await;
				
				match &self.user
				{
					Some(user) => {
						match &user.id
						{
							Some(id) => {
								if let Some(newId) = self.userGetClientId(&id.to_string())
								{
									self.id = newId;
									
									let data: HashMap<String, String> = vec![
										("clientId".to_string(), self.id.to_string()),
										("username".to_string(), user.name.to_owned()),
									].into_iter().collect();
									
									self.queueCommand(
										self.id,
										Commands::AuthenticateSuccess,
										Some(data),
										None
									)?;
									
									self.queueBroadcast(format!("{} ({}) connected!", user.name, self.id))?;
								}
							},
							
							None => self.queueCommandSimple(self.id, Commands::AuthenticateFail)?,
						}
					},
					
					None => self.queueCommandSimple(self.id, Commands::AuthenticateFail)?,
				}
			}
		}
		
		return Ok(());
	}
	
	async fn handleBroadcastGetRequest(&self, command: Command) -> Result<()>
	{
		if let Some(start) = parseDateTime(command.Data.get("start"))
		{
			if let Some(end) = parseDateTime(command.Data.get("end"))
			{
				self.queueExistingMessages(start.naive_utc(), end.naive_utc()).await?;
			}
		}
		
		return Ok(());
	}
	
	async fn handleBroadcastSend(&self, command: Command) -> Result<()>
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
	
	async fn handleScene2dRequest(&self, _command: Command) -> Result<()>
	{
		let image: Image = loadAsset("BackgroundPlaceholder.png".into())?;
		
		let data: HashMap<String, String> = vec![
			("height".into(), "600".into()),
			("width".into(), "900".into()),
		].into_iter().collect();
		
		let binaryData: HashMap<String, String> = vec![
			("background".into(), BASE64_STANDARD.encode(image.bytes()?)),
		].into_iter().collect();
		
		self.queueCommand(
			self.id,
			Commands::Scene2DResponse,
			Some(data),
			Some(binaryData)
		)?;
		
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
	
	fn queueCommandSimple(&self, id: i64, command: Commands) -> Result<()>
	{
		return self.queueCommand(id, command, None, None);
	}
	
	fn queueCommand(&self,
		id: i64,
		command: Commands,
		data: Option<HashMap<String, String>>,
		binaryData: Option<HashMap<String, String>>
	) -> Result<()>
	{
		if let Ok(queue) = getMessageQueue().lock()
		{
			queue.queueCommand(id, command, data, binaryData)?;
		}
		
		return Ok(());
	}
	
	async fn queueExistingMessages(&self, start: NaiveDateTime, end: NaiveDateTime) -> Result<()>
	{
		let messages = dao::messageFindByDateRange(start, end).await?;
		if !messages.is_empty()
		{
			let users = dao::userGetAll().await?;
			
			for m in messages
			{
				let username = match m.userId
				{
					None => String::default(),
					Some(userId) => match users.iter()
						.find(|u| u.id.as_ref().is_some_and(|t| t == &userId))
					{
						None => String::default(),
						Some(user) => user.name.to_owned(),
					},
				};
				
				let data: HashMap<String, String> = vec![
					(
						"text".to_string(),
						match username.is_empty()
						{
							true => m.text.to_owned(),
							false => format!("{}: {}", username, m.text),
						}
					),
				].into_iter().collect();
				
				self.queueCommand(
					self.id.to_owned(),
					Commands::BroadcastResponse,
					Some(data),
					None
				)?;
			}
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
			Err(e) => {
				error!("Error reading messages for client id {}: {:?}", self.id, e);
				vec![]
			},
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
	
	async fn userFindOrCreate(&self, username: &String) -> Option<User>
	{
		return match dao::userFind(username.clone()).await
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
					
					match dao::userCreate(Some(content)).await
					{
						Ok(opt) => opt,
						
						Err(e) => {
							error!("Error creating new user with username '{}': {:?}", username, e);
							None
						},
					}
				},
				
				Some(u) => Some(u),
			},
		};
	}
}
