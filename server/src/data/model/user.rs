use ::serde::{Deserialize, Serialize};
use ::surrealdb::sql::Thing;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User
{
	pub id: Thing,
	pub label: Option<String>,
	pub name: String,
}

impl User
{
	pub const Name: &'static str = "user";
}
