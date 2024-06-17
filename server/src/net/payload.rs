use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};
use super::enums::Commands;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct Command
{
	pub Data: HashMap<String, String>,
	pub Id: i64,
	pub Timestamp: i64,
	pub Type: Commands,
}
