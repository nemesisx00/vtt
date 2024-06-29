use std::collections::HashMap;
use ::chrono::Utc;
use ::serde::{Deserialize, Serialize};
use super::commands::Commands;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Command
{
	/// Binary data must be base64 encoded before transmission to clients
	pub BinaryData: HashMap<String, String>,
	pub Data: HashMap<String, String>,
	pub Id: i64,
	pub Timestamp: i64,
	pub Type: Commands,
}

impl Default for Command
{
	fn default() -> Self
	{
		return Self
		{
			BinaryData: HashMap::default(),
			Data: HashMap::default(),
			Id: 0,
			Timestamp: Utc::now().timestamp(),
			Type: Commands::None,
		};
	}
}
