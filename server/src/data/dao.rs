use ::anyhow::Result;
use ::chrono::NaiveDateTime;
use ::diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use super::db::getDatabase;
use super::model::{Message, NewMessage, NewUser, User};
use super::schema;
use super::schema::messages::dsl::messages;
use super::schema::users::dsl::users;

pub async fn messageCreate(newMessage: NewMessage) -> Result<Option<Message>>
{
	let mut db = getDatabase().lock().await;
	
	let message = match db.connection
	{
		None => None,
		Some(ref mut conn) => {
			Some(diesel::insert_into(schema::messages::table)
				.values(newMessage)
				.returning(Message::as_returning())
				.get_result(conn)?)
		}
	};
	
	return Ok(message);
}

pub async fn messageFindByDateRange(start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<Message>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => vec![],
		Some(ref mut conn) => messages
			.filter(super::schema::messages::dsl::timestamp.ge(start))
			.filter(super::schema::messages::dsl::timestamp.le(end))
			.load(conn)?
	};
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn messageFindByUser(userId: i32) -> Result<Vec<Message>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => vec![],
		Some(ref mut conn) => messages
			.filter(super::schema::messages::dsl::userId.eq(userId))
			.load(conn)?
	};
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn messageGetAll() -> Result<Vec<Message>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => vec![],
		Some(ref mut conn) => messages.load(conn)?
	};
	
	return Ok(result);
}

pub async fn userCreate(newUser: NewUser) -> Result<Option<User>>
{
	let mut db = getDatabase().lock().await;
	
	let user: Option<User> = match db.connection
	{
		None => None,
		Some(ref mut conn) => Some(diesel::insert_into(schema::users::table)
			.values(newUser)
			.returning(User::as_returning())
			.get_result(conn)?)
	};
	
	return Ok(user);
}

#[allow(dead_code)]
pub async fn userDelete(user: User) -> Result<()>
{
	let mut db = getDatabase().lock().await;
	
	if let Some(ref mut conn) = db.connection
	{
		diesel::delete(users)
			.filter(super::schema::users::dsl::id.eq(user.id))
			.execute(conn)?;
	}
	
	return Ok(());
}

pub async fn userFind(username: String) -> Result<Option<User>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => None,
		Some(ref mut conn) => users
			.filter(super::schema::users::dsl::name.eq(username))
			.first(conn)
			.optional()?
	};
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn userGet(id: i32) -> Result<Option<User>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => None,
		Some(ref mut conn) => users
			.find(id)
			.first(conn)
			.optional()?
	};
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn userGetAll() -> Result<Vec<User>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => vec![],
		Some(ref mut conn) => users.load(conn)?
	};
	
	return Ok(result);
}

#[allow(dead_code)]
pub async fn userUpdate(user: User) -> Result<Option<User>>
{
	let mut db = getDatabase().lock().await;
	
	let result = match db.connection
	{
		None => None,
		Some(ref mut conn) => Some(diesel::update(users.filter(super::schema::users::dsl::id.eq(user.id)))
			.set((
				super::schema::users::dsl::name.eq(user.name),
				super::schema::users::dsl::label.eq(user.label)
			))
			.returning(User::as_returning())
			.get_result(conn)?)
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
			db.dropAll().expect("Failed to drop all tables");
			db.initialize().expect("Failed to initialize database");
		}
		
		let username = "myusername".to_string();
		
		let newUser = NewUser
		{
			name: username.to_owned(),
			..Default::default()
		};
		
		println!("new user: {:?}", newUser);
		let mut user = dao::userCreate(newUser).await.expect("Error creating test user");
		assert!(user.is_some());
		
		if let Some(u) = user.as_mut()
		{
			let got = dao::userGet(u.id).await.expect("Error getting user by id");
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
		
		let allUsers = dao::userGetAll().await.expect("Error getting all users");
		assert!(!allUsers.is_empty());
		let first = allUsers.first();
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
