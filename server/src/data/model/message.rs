use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Thing};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Message
{
	pub id: Option<Thing>,
	pub text: String,
	pub timestamp: i64,
	pub userId: Option<Thing>,
}

impl IntoResource<Resource> for Message
{
	fn into_resource(self) -> Result<Resource>
	{
		let mut obj = Object::default();
		
		if let Some(id) = &self.id
		{
			obj.insert("id".into(), id.to_owned().into());
		}
		
		obj.insert("text".into(), self.text.to_owned().into());
		obj.insert("timestamp".into(), self.timestamp.to_owned().into());
		
		if let Some(userId) = &self.userId
		{
			obj.insert("userId".into(), userId.to_owned().into());
		}
		
		return Ok(obj.into());
	}
}

impl Message
{
	pub const ResourceName: &'static str = "message";
}
