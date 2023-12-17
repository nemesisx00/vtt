#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub fn getMessageQueue() -> &'static Mutex<MessageQueue>
{
	static MessageQueueLock: OnceLock<Mutex<MessageQueue>> = OnceLock::new();
	return MessageQueueLock.get_or_init(|| Mutex::new(MessageQueue::default()));
}

#[derive(Clone, Default)]
pub struct MessageQueue
{
	queue: RefCell<HashMap<i64, Vec<String>>>,
}

unsafe impl Send for MessageQueue {}

impl MessageQueue
{
	pub fn isEmpty(&self) -> bool
	{
		return self.queue.borrow().is_empty();
	}
	
	/**
	Queue a new message for transmission to every registered client.
	*/
	pub fn queueBroadcast(&self, message: String)
	{
		for (_, list) in self.queue.borrow_mut().iter_mut()
		{
			list.push(message.to_owned());
		}
	}
	
	/**
	Queue a new message for transmission to a given `id`.
	*/
	pub fn queueMessage(&self, id: i64, message: String)
	{
		let mut q = self.queue.borrow_mut();
		
		//Just in case
		if !q.contains_key(&id)
		{
			q.insert(id, vec![]);
		}
		
		if let Some(map) = q.get_mut(&id)
		{
			map.push(message);
		}
	}
	
	/**
	Read the current list of queued messages for a given `id`.
	
	This method removes the messages from the queue before returning the list.
	*/
	pub fn readMessages(&self, id: i64) -> Vec<String>
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
	pub fn removeId(&self, id: i64) -> Vec<String>
	{
		let mut q = self.queue.borrow_mut();
		
		let mut output = vec![];
		if let Some(list) = q.remove(&id)
		{
			output = list;
		}
		return output;
	}
}
