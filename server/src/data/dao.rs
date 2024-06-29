use std::future::IntoFuture;
use ::anyhow::Result;
use ::chrono::NaiveDateTime;
use ::surrealdb::sql::Thing;
use super::db::getDatabase;
use super::model::{Message, User};

const ParameterEnd: &'static str = "end";
const ParameterStart: &'static str = "start";
const ParameterUserId: &'static str = "userId";
const ParameterUsername: &'static str = "username";

#[allow(dead_code)]
const SelectByIdTemplate: &'static str = "SELECT * FROM ";

const SelectMessageByDateRange: &'static str = r#"SELECT *
FROM message
WHERE timestamp >= $start
	AND timestamp <= $end
ORDER BY timestamp ASC"#;

const SelectMessageByUserId: &'static str = r#"SELECT *
FROM message
WHERE userId = $userId"#;

const SelectUserByName: &'static str = r#"SELECT *
FROM user
WHERE name = $username"#;

pub async fn messageCreate(content: Option<Message>) -> Result<Option<Message>>
{
	let db = getDatabase().lock().await;
	
	let result: Vec<Message> = db.instance
		.create(Message::ResourceName)
		.content(content)
		.await?;
	
	let message = match result.first()
	{
		Some(msg) => Some(msg.clone()),
		None => None,
	};
	
	return Ok(message);
}

pub async fn messageFindByDateRange(start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<Message>>
{
	let db = getDatabase().lock().await;
	
	let result = db.instance
		.query(SelectMessageByDateRange)
		.bind((ParameterStart, start.and_utc().timestamp()))
		.bind((ParameterEnd, end.and_utc().timestamp()))
		.await?
		.take(0)?;
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn messageFindByUser(userId: Thing) -> Result<Vec<Message>>
{
	let db = getDatabase().lock().await;
	
	let result = db.instance
		.query(SelectMessageByUserId)
		.bind((ParameterUserId, userId))
		.await?
		.take(0)?;
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn messageGetAll() -> Result<Vec<Message>>
{
	let db = getDatabase().lock().await;
	
	let result = db.instance
		.select(Message::ResourceName)
		.await?;
	
	return Ok(result);
}

pub async fn userCreate(content: Option<User>) -> Result<Option<User>>
{
	let db = getDatabase().lock().await;
	
	let result: Vec<User> = db.instance
		.create(User::ResourceName)
		.content(content)
		.await?;
	
	let user = match result.first()
	{
		Some(u) => Some(u.clone()),
		None => None,
	};
	
	return Ok(user);
}

#[allow(dead_code)]
pub async fn userDelete(user: User) -> Result<()>
{
	let db = getDatabase().lock().await;
	
	if let Some(thing) = user.id
	{
		let _: Option<User> = db.instance
			.delete(thing)
			.into_future()
			.await?;
	}
	
	return Ok(());
}

pub async fn userFind(username: String) -> Result<Option<User>>
{
	let db = getDatabase().lock().await;
	
	let result = db.instance
		.query(SelectUserByName)
		.bind((ParameterUsername, username))
		.await?
		.take(0)?;
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn userGet(id: Thing) -> Result<Option<User>>
{
	let db = getDatabase().lock().await;
	
	let result = db.instance
		.query(format!("{}{}", SelectByIdTemplate, id))
		.await?
		.take(0)?;
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn userGetAll() -> Result<Vec<User>>
{
	let db = getDatabase().lock().await;
	
	let result = db.instance
		.select(User::ResourceName)
		.await?;
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn userUpdate(user: User) -> Result<Option<User>>
{
	
	let result = match &user.id
	{
		Some(thing) => {
			let content = User
			{
				label: user.label.to_owned(),
				name: user.name.to_owned(),
				..Default::default()
			};
			
			let db = getDatabase().lock().await;
			
			db.instance
				.update(thing)
				.content(content)
				.await?
		},
		None => None,
	};
	
	return Ok(result);
}

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::data::dao;
	
	#[tokio::test]
	async fn createGetUpdateUser()
	{
		{
			let mut db = getDatabase().lock().await;
			db.initialize().await.expect("Failed to initialize database");
		}
		
		let username = "myusername".to_string();
		
		let content = User
		{
			name: username.to_owned(),
			..Default::default()
		};
		
		let mut user = dao::userCreate(Some(content)).await.expect("Error creating test user");
		assert!(user.is_some());
		
		if let Some(u) = user.as_mut()
		{
			let got = dao::userGet(u.id.clone().unwrap()).await.expect("Error getting user by id");
			assert!(got.is_some_and(|g| &g == u && g.label.is_none()));
			
			u.label = Some("My Username".into());
			
			let userClone = u.clone();
			assert!(userClone.label.is_some());
			assert_eq!(userClone.label, u.label);
			
			let updated = dao::userUpdate(userClone).await.expect("Error updating user");
			assert!(updated.is_some());
			let updatedUser = updated.unwrap();
			assert_eq!(updatedUser.name, u.name);
			assert_eq!(updatedUser.label, u.label);
		}
		
		let users = dao::userGetAll().await.expect("Error getting all users");
		assert!(!users.is_empty());
		let first = users.first();
		assert!(first.is_some());
		let u = first.unwrap();
		assert_eq!(Some(u.clone()), user);
		
		let found = dao::userFind(username.to_owned()).await.expect("Error finding user by username");
		assert!(found.is_some());
		let foundUser = found.unwrap();
		assert_eq!(foundUser.name, username);
		
		let deleted = dao::userDelete(user.unwrap()).await;
		assert!(deleted.is_ok());
		
		let usersAgain = dao::userGetAll().await.expect("Error getting all users again");
		assert!(usersAgain.is_empty());
	}
}
