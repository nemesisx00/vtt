use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use ::anyhow::Result;
use ::serde::Serialize;
use ::serde::de::DeserializeOwned;
use ::surrealdb::Surreal;
use ::surrealdb::engine::local::{Db, Mem, RocksDb};
use crate::config::{localDataPath, Config, ConfigDatabase};
use super::dbtype::DatabaseType;

#[derive(Debug)]
pub struct Database
{
	pub databaseConfig: ConfigDatabase,
	instance: RefCell<Option<Surreal<Db>>>,
}

impl Database
{
	pub fn from(config: &Config) -> Self
	{
		return Self::new(config.database.clone());
	}
	
	pub fn new(config: ConfigDatabase) -> Self
	{
		return Self
		{
			databaseConfig: config,
			instance: RefCell::new(None),
		};
	}
	
	pub async fn create<T>(&self, resource: String, content: Option<HashMap<String, String>>) -> Result<Vec<T>>
		where T: Serialize + DeserializeOwned
	{
		self.open().await?;
		
		let mut created = vec![];
		if let Some(db) = self.instance.borrow().as_ref()
		{
			created = match content
			{
				None => db.create(resource).await?,
				Some(c) => db.create(resource)
					.content(c)
					.await?,
			};
		}
		
		return Ok(created);
	}
	
	pub async fn queryOne<T>(&self, query: String) -> Result<Option<T>>
		where T: DeserializeOwned
	{
		self.open().await?;
		
		let mut result: Option<T> = None;
		if let Some(db) = self.instance.borrow().as_ref()
		{
			result = db.query(query)
				.await?
				.take(0)?;
		}
		
		return Ok(result)
	}
	
	#[allow(dead_code)]
	pub async fn queryAll<T>(&self, query: String) -> Result<Vec<T>>
		where T: DeserializeOwned
	{
		self.open().await?;
		
		let mut result: Vec<T> = vec![];
		if let Some(db) = self.instance.borrow().as_ref()
		{
			result = db.query(query)
				.await?
				.take(0)?;
		}
		
		return Ok(result)
	}
	
	pub async fn select<T>(&self, resource: String) -> Result<Vec<T>>
		where T: Serialize + DeserializeOwned
	{
		self.open().await?;
		
		let mut result = vec![];
		if let Some(db) = self.instance.borrow().as_ref()
		{
			result = db.select(resource).await?;
		}
		
		return Ok(result);
	}
	
	pub async fn update<T>(&self, resourceId: (String, String), record: T) -> Result<Option<T>>
		where T: Serialize + DeserializeOwned
	{
		self.open().await?;
		
		let mut result: Option<T> = None;
		if let Some(db) = self.instance.borrow().as_ref()
		{
			result = db.update(resourceId)
				.content(record)
				.await?;
		}
		
		return Ok(result);
	}
	
	async fn open(&self) -> Result<()>
	{
		if self.instance.borrow().as_ref().is_none()
		{
			let mut filePath = self.databaseConfig.path.clone();
			if let Some(dir) = localDataPath()
			{
				let buf = PathBuf::from(dir)
					.join(filePath.clone());
				
				if let Ok(p) = buf.into_os_string().into_string()
				{
					filePath = p;
				}
			}
			
			let db = match self.databaseConfig.databaseType
			{
				DatabaseType::Memory => Surreal::new::<Mem>(()).await?,
				DatabaseType::RocksDB => Surreal::new::<RocksDb>(filePath).await?,
			};
			
			db.use_ns(self.databaseConfig.namespace.to_owned())
				.use_db(self.databaseConfig.name.to_owned())
				.await?;
			
			*self.instance.borrow_mut() = Some(db);
		}
		
		return Ok(());
	}
}
