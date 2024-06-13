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
	instance: Surreal<Db>,
}

impl Database
{
	pub async fn from(config: Config) -> Result<Self>
	{
		return Self::new(config.database.clone()).await;
	}
	
	pub async fn new(config: ConfigDatabase) -> Result<Self>
	{
		return Ok(Self
		{
			instance: Self::instantiate(config).await?,
		});
	}
	
	pub async fn create<T>(&self, resource: String, content: Option<HashMap<String, String>>) -> Result<Vec<T>>
		where T: Serialize + DeserializeOwned
	{
		let created = match content
		{
			None => self.instance.create(resource).await?,
			Some(c) =>self.instance.create(resource)
				.content(c)
				.await?,
		};
		
		return Ok(created);
	}
	
	pub async fn queryOne<T>(&self, query: String) -> Result<Option<T>>
		where T: DeserializeOwned
	{
		let result = self.instance.query(query)
			.await?
			.take(0)?;
		
		return Ok(result)
	}
	
	pub async fn queryOneArgs<T>(&self, query: String, bindings: impl Serialize) -> Result<Option<T>>
		where T: DeserializeOwned
	{
		let result = self.instance.query(query)
			.bind(bindings)
			.await?
			.take(0)?;
		
		return Ok(result);
	}
	
	#[allow(dead_code)]
	pub async fn queryAll<T>(&self, query: String) -> Result<Vec<T>>
		where T: DeserializeOwned
	{
		let result = self.instance.query(query)
			.await?
			.take(0)?;
		
		return Ok(result)
	}
	
	pub async fn select<T>(&self, resource: String) -> Result<Vec<T>>
		where T: Serialize + DeserializeOwned
	{
		let result: Vec<T> = self.instance.select(resource).await?;
		
		return Ok(result);
	}
	
	pub async fn update<T>(&self, resourceId: (String, String), record: T) -> Result<Option<T>>
		where T: Serialize + DeserializeOwned
	{
		let result = self.instance.update(resourceId)
			.content(record)
			.await?;
		
		return Ok(result);
	}
	
	async fn instantiate(config: ConfigDatabase) -> Result<Surreal<Db>>
	{
		let mut filePath = config.path.clone();
		if let Some(dir) = localDataPath()
		{
			let buf = PathBuf::from(dir)
				.join(filePath.clone());
			
			if let Ok(p) = buf.into_os_string().into_string()
			{
				filePath = p;
			}
		}
		
		let db = match config.databaseType
		{
			DatabaseType::Memory => Surreal::new::<Mem>(()).await?,
			DatabaseType::RocksDB => Surreal::new::<RocksDb>(filePath).await?,
		};
		
		db.use_ns(config.namespace.to_owned())
			.use_db(config.name.to_owned())
			.await?;
		
		return Ok(db);
	}
}
