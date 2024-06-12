use std::collections::HashMap;

use ::anyhow::Result;
use ::surrealdb::sql::Thing;
use crate::config::Config;
use super::db::Database;
use super::model::User;

const SelectByIdTemplate: &'static str = "SELECT * FROM ";

pub struct DataLayer
{
	db: Database,
}

impl DataLayer
{
	pub fn new(config: &Config) -> Self
	{
		return Self
		{
			db: Database::from(config),
		}
	}
	
	pub async fn userCreate(&self, content: Option<HashMap<String, String>>) -> Result<Option<User>>
	{
		let result: Vec<User> = self.db.create(User::Name.into(), content).await?;
		
		let mut user = None;
		if let Some(u) = result.first()
		{
			user = Some(u.clone());
		}
		
		return Ok(user);
	}
	
	pub async fn userGet(&self, id: Thing) -> Result<Option<User>>
	{
		let result = self.db.queryOne(format!("{}{}", SelectByIdTemplate, id)).await?;
		return Ok(result);
	}
	
	pub async fn userGetAll(&self) -> Result<Vec<User>>
	{
		let result = self.db.select(User::Name.into()).await?;
		return Ok(result);
	}
	
	pub async fn userUpdate(&self, user: User) -> Result<Option<User>>
	{
		let result = self.db.update((User::Name.into(), user.id.id.to_string()), user).await?;
		return Ok(result);
	}
}

#[cfg(test)]
mod tests
{
	use crate::config::Config;
	use super::*;
	
	#[tokio::test]
	async fn createGetUpdateUser()
	{
		let config = Config::getTestConfig();
		let dal = DataLayer::new(&config);
		
		let mut content = HashMap::<String, String>::default();
		content.insert("name".into(), "nemesis".into());
		
		let mut user = dal.userCreate(Some(content)).await.unwrap();
		assert!(user.is_some());
		
		if let Some(u) = user.as_mut()
		{
			let got = dal.userGet(u.id.clone()).await.unwrap();
			assert!(got.is_some_and(|g| &g == u && g.label.is_none()));
			
			u.label = Some("Nemesis".into());
			
			let updated = dal.userUpdate(u.clone()).await.unwrap();
			assert!(updated.is_some_and(|up| up.name == u.name && up.label.is_some()));
		}
		
		let users = dal.userGetAll().await.unwrap();
		assert!(!users.is_empty());
		assert!(users.first().is_some_and(|u| Some(u.clone()) == user));
	}
}
