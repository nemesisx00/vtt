use ::serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config
{
	pub database: ConfigDatabase,
	pub network: ConfigNetwork,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ConfigDatabase
{
	pub connectionString: String,
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
