use ::serde::Deserialize;
use crate::data::DatabaseType;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config
{
	pub assets: ConfigAssets,
	pub database: ConfigDatabase,
	pub network: ConfigNetwork,
}

impl Config
{
	#[cfg(test)]
	pub fn getTestConfig() -> Self
	{
		return Self
		{
			assets: ConfigAssets
			{
				path: None,
			},
			
			database: ConfigDatabase
			{
				databaseType: DatabaseType::Memory,
				name: "vtt".into(),
				namespace: "vtt".into(),
				path: "data".into(),
			},
			
			network: ConfigNetwork
			{
				ip: "127.0.0.1".into(),
				port: 8080,
			},
		};
	}
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ConfigAssets
{
	pub path: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ConfigDatabase
{
	pub databaseType: DatabaseType,
	pub name: String,
	pub namespace: String,
	pub path: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ConfigNetwork
{
	pub ip: String,
	pub port: u16,
}

impl ConfigNetwork
{
	/**
	Return the full address in form `<ip>:<port>`.
	*/
	pub fn fullAddress(&self) -> String
	{
		return format!("{}:{}", self.ip, self.port);
	}
}
