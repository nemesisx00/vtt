use std::path::PathBuf;
use std::sync::OnceLock;
use ::anyhow::Result;
use diesel::RunQueryDsl;
use ::diesel::{Connection, SqliteConnection};
use ::tokio::sync::Mutex;
use crate::config::localDataPath;
use crate::data::model::{CreateTable_ImageAssets, CreateTable_Messages,
	CreateTable_Scenes2D, CreateTable_Users, DropTable_ImageAssets,
	DropTable_Messages, DropTable_Scenes2D, DropTable_Users};
use crate::getConfig;

pub fn getDatabase() -> &'static Mutex<Database>
{
	static DbLock: OnceLock<Mutex<Database>> = OnceLock::new();
	return DbLock.get_or_init(|| Mutex::new(Database::default()));
}

pub struct Database
{
	pub connection: Option<SqliteConnection>,
}

impl Default for Database
{
	fn default() -> Self
	{
		return Self
		{
			connection: None,
		};
	}
}

impl Database
{
	#[allow(unused)]
	pub fn dropAll(&mut self) -> Result<()>
	{
		if let Some(ref mut conn) = self.connection
		{
			diesel::sql_query(DropTable_ImageAssets).execute(conn)?;
			diesel::sql_query(DropTable_Messages).execute(conn)?;
			diesel::sql_query(DropTable_Scenes2D).execute(conn)?;
			diesel::sql_query(DropTable_Users).execute(conn)?;
		}
		
		return Ok(());
	}
	
	pub fn initialize(&mut self) -> Result<()>
	{
		if self.connection.is_none()
		{
			let config = getConfig();
			
			let mut filePath = config.database.path.clone();
			if let Some(dir) = localDataPath()
			{
				let buf = PathBuf::from(dir)
					.join(filePath.clone());
				
				if let Ok(p) = buf.into_os_string().into_string()
				{
					filePath = p;
				}
			}
			
			self.connection = Some(SqliteConnection::establish(&filePath)?);
		}
		
		if let Some(ref mut conn) = self.connection
		{
			diesel::sql_query(CreateTable_ImageAssets).execute(conn)?;
			diesel::sql_query(CreateTable_Messages).execute(conn)?;
			diesel::sql_query(CreateTable_Scenes2D).execute(conn)?;
			diesel::sql_query(CreateTable_Users).execute(conn)?;
		}
		
		return Ok(());
	}
}
