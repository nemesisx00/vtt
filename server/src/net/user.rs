use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub fn getUserManager() -> &'static Mutex<UserManager>
{
	static UserManagerLock: OnceLock<Mutex<UserManager>> = OnceLock::new();
	return UserManagerLock.get_or_init(|| Mutex::new(UserManager::default()));
}

#[derive(Clone, Default)]
pub struct UserManager
{
	nextId: RefCell<i64>,
	users: RefCell<HashMap<String, i64>>,
}

unsafe impl Send for UserManager {}

impl UserManager
{
	pub fn getClientId(&self, username: &String) -> Option<i64>
	{
		let mut users = self.users.borrow_mut();
		
		if !users.contains_key(username)
		{
			users.insert(username.to_owned(), self.getNextId());
		}
		
		let id = users.get(username)?;
		return Some(*id);
	}
	
	#[allow(dead_code)]
	pub fn getUserId(&self, clientId: i64) -> Option<String>
	{
		let mut username = None;
		
		for (user, client) in self.users.borrow().iter()
		{
			if client == &clientId
			{
				username = Some(user.to_owned());
			}
		}
		
		return username;
	}
	
	fn getNextId(&self) -> i64
	{
		let mut nextId = self.nextId.borrow_mut();
		*nextId += 1;
		return nextId.clone();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	#[test]
	fn multipleUsers()
	{
		let username1 = "username1".to_string();
		let username2 = "2emanresu".to_string();
		
		let manager = UserManager::default();
		
		let clientId1 = manager.getClientId(&username1);
		assert!(clientId1.is_some_and(|id| id == 1));
		
		let clientId2 = manager.getClientId(&username2);
		assert!(clientId2.is_some_and(|id| id == 2));
		
		let clientId3 = manager.getClientId(&username1);
		assert!(clientId3.is_some_and(|id| id == 1));
		
		let clientId4 = manager.getClientId(&username2);
		assert!(clientId4.is_some_and(|id| id == 2));
	}
}
