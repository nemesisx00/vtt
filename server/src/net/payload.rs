use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Broadcast
{
	pub id: i64,
	pub text: String,
}

impl Broadcast
{
	pub const Name: &'static str = "Broadcast";
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct ClientIdentity
{
	pub id: i64,
}

impl ClientIdentity
{
	pub const Name: &'static str = "ClientIdentity";
}
