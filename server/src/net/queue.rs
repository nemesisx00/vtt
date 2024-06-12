use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use ::anyhow::Result;
use super::enums::Commands;
use super::payload::Command;

pub fn getMessageQueue() -> &'static Mutex<MessageQueue>
{
	static MessageQueueLock: OnceLock<Mutex<MessageQueue>> = OnceLock::new();
	return MessageQueueLock.get_or_init(|| Mutex::new(MessageQueue::default()));
}

#[derive(Clone, Default)]
pub struct MessageQueue
{
	queue: RefCell<HashMap<i64, Vec<Command>>>,
}

unsafe impl Send for MessageQueue {}

impl MessageQueue
{
	#[allow(dead_code)]
	pub fn isEmpty(&self) -> bool
	{
		return self.queue.borrow().is_empty();
	}
	
	/**
	Queue a new message for transmission to every registered client.
	*/
	pub fn queueBroadcast(&self, message: String) -> Result<()>
	{
		let mut datamap = HashMap::<String, String>::default();
		datamap.insert("text".into(), message);
		return self.queueCommand(-1, Commands::BroadcastReceive, Some(datamap));
	}
	
	pub fn queueCommand(&self, id: i64, command: Commands, data: Option<HashMap<String, String>>) -> Result<()>
	{
		let datamap = match data
		{
			None => HashMap::default(),
			Some(map) => map,
		};
		
		self.queueMessage(Command
		{
			Id: id,
			Type: command,
			Data: datamap
		});
		
		return Ok(());
	}
	
	/**
	Read the current list of queued messages for a given `id`.
	
	This method removes the messages from the queue before returning the list.
	*/
	pub fn readMessages(&self, id: i64) -> Vec<Command>
	{
		let mut messages = vec![];
		
		let mut q = self.queue.borrow_mut();
		if let Some(msgs) = q.get_mut(&id)
		{
			for msg in msgs.iter()
			{
				messages.push(msg.to_owned());
			}
			msgs.clear();
		}
		
		return messages;
	}
	
	/**
	Register a given `id`.
	
	Ensures that the given `id` has an associated message list in the queue.
	*/
	#[allow(dead_code)]
	pub fn registerId(&self, id: i64)
	{
		let mut q = self.queue.borrow_mut();
		if !q.contains_key(&id)
		{
			q.insert(id, vec![]);
		}
	}
	
	/**
	Remove the message list from the queue for a given `id`.
	*/
	pub fn removeId(&self, id: i64) -> Vec<Command>
	{
		let mut q = self.queue.borrow_mut();
		
		let mut output = vec![];
		if let Some(list) = q.remove(&id)
		{
			output = list;
		}
		return output;
	}
	
	/**
	Queue a new message for transmission to a given `id`.
	*/
	fn queueMessage(&self, command: Command)
	{
		let mut q = self.queue.borrow_mut();
		
		//Just in case
		if !q.contains_key(&command.Id)
		{
			q.insert(command.Id, vec![]);
		}
		
		if let Some(map) = q.get_mut(&command.Id)
		{
			map.push(command);
		}
	}
}
