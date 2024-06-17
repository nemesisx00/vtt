use std::path::PathBuf;
use std::sync::OnceLock;
use ::anyhow::Result;
use ::surrealdb::Surreal;
use ::surrealdb::engine::local::{Db, Mem, RocksDb};
use ::tokio::sync::Mutex;
use crate::config::{localDataPath, ConfigDatabase};
use super::dbtype::DatabaseType;

pub fn getDatabase() -> &'static Mutex<Database>
{
	static DbLock: OnceLock<Mutex<Database>> = OnceLock::new();
	return DbLock.get_or_init(|| Mutex::new(Database::default()));
}

#[derive(Debug)]
pub struct Database
{
	initialized: bool,
	pub instance: Surreal<Db>,
}

impl Default for Database
{
	fn default() -> Self
	{
		return Self
		{
			initialized: false,
			instance: Surreal::init(),
		};
	}
}

impl Database
{
	pub async fn initialize(&mut self, config: ConfigDatabase) -> Result<()>
	{
		if !self.initialized
		{
			self.instance = match config.databaseType
			{
				DatabaseType::Memory => Surreal::new::<Mem>(()).await?,
				
				DatabaseType::RocksDB => {
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
					
					Surreal::new::<RocksDb>(filePath).await?
				},
			};
			
			self.instance
				.use_ns(config.namespace.to_owned())
				.use_db(config.name.to_owned())
				.await?;
		}
		
		return Ok(());
	}
}
