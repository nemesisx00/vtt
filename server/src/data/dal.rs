use std::collections::HashMap;
use std::sync::OnceLock;
use ::anyhow::Result;
use ::surrealdb::sql::Thing;
use ::tokio::sync::Mutex;
use crate::config::Config;
use super::db::Database;
use super::model::User;

pub fn getDao() -> &'static Mutex<DataLayer>
{
	static DaoLock: OnceLock<Mutex<DataLayer>> = OnceLock::new();
	return DaoLock.get_or_init(|| Mutex::new(DataLayer::default()));
}

#[allow(dead_code)]
const SelectByIdTemplate: &'static str = "SELECT * FROM ";
const SelectUserByNameTemplate: &'static str = "SELECT * FROM user WHERE name = $username";

const ParameterUsername: &'static str = "username";

pub struct DataLayer
{
	db: Database,
}

impl Default for DataLayer
{
	fn default() -> Self
	{
		return Self { db: Database::default() };
	}
}

impl DataLayer
{
	pub async fn initialize(&mut self, config: Config) -> Result<()>
	{
		self.db.initialize(config.database).await?;
		return Ok(());
	}
	
	pub async fn userCreate(&self, content: Option<HashMap<String, String>>) -> Result<Option<User>>
	{
		let result: Vec<User> = self.db.create(User::ResourceName.into(), content).await?;
		
		let mut user = None;
		if let Some(u) = result.first()
		{
			user = Some(u.clone());
		}
		
		return Ok(user);
	}
	
	pub async fn userFind(&self, username: String) -> Result<Option<User>>
	{
		let result = self.db.queryOneArgs(
			SelectUserByNameTemplate.into(),
			(ParameterUsername, username)
		).await?;
		
		return Ok(result);
	}
	
	#[allow(dead_code)]
	pub async fn userGet(&self, id: Thing) -> Result<Option<User>>
	{
		let result = self.db.queryOne(format!("{}{}", SelectByIdTemplate, id)).await?;
		return Ok(result);
	}
	
	#[allow(dead_code)]
	pub async fn userGetAll(&self) -> Result<Vec<User>>
	{
		let result = self.db.select(User::ResourceName.into()).await?;
		return Ok(result);
	}
	
	#[allow(dead_code)]
	pub async fn userUpdate(&self, user: User) -> Result<Option<User>>
	{
		let result = self.db.update((User::ResourceName.into(), user.id.id.to_string()), user).await?;
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
		let mut dao = DataLayer::default();
		dao.initialize(config).await.unwrap();
		
		let username = "nemesis".to_string();
		
		let mut content = HashMap::<String, String>::default();
		content.insert("name".into(), username.to_owned());
		
		let mut user = dao.userCreate(Some(content)).await.unwrap();
		assert!(user.is_some());
		
		if let Some(u) = user.as_mut()
		{
			let got = dao.userGet(u.id.clone()).await.unwrap();
			assert!(got.is_some_and(|g| &g == u && g.label.is_none()));
			
			u.label = Some("Nemesis".into());
			
			let updated = dao.userUpdate(u.clone()).await.unwrap();
			assert!(updated.is_some_and(|up| up.name == u.name && up.label.is_some()));
		}
		
		let users = dao.userGetAll().await.unwrap();
		assert!(!users.is_empty());
		assert!(users.first().is_some_and(|u| Some(u.clone()) == user));
		
		let found = dao.userFind(username.to_owned()).await.unwrap();
		assert!(found.is_some_and(|u| u.name == username));
	}
}
