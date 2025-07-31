use ::serde::Deserialize;

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
				path: "testData.sqlite".into(),
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
