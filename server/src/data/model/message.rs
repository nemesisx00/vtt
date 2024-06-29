use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Thing};

const Property_Id: &'static str = "id";
const Property_Text: &'static str = "text";
const Property_Timestamp: &'static str = "timestamp";
const Property_UserId: &'static str = "userId";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Message
{
	pub id: Option<Thing>,
	pub text: String,
	pub timestamp: i64,
	pub userId: Option<Thing>,
}

impl Into<Object> for Message
{
	fn into(self) -> Object
	{
		let mut obj = Object::default();
		
		if let Some(id) = &self.id
		{
			obj.insert(Property_Id.into(), id.to_owned().into());
		}
		
		obj.insert(Property_Text.into(), self.text.to_owned().into());
		obj.insert(Property_Timestamp.into(), self.timestamp.to_owned().into());
		
		if let Some(userId) = &self.userId
		{
			obj.insert(Property_UserId.into(), userId.to_owned().into());
		}
		
		return obj;
	}
}

impl IntoResource<Resource> for Message
{
	fn into_resource(self) -> Result<Resource>
	{
		let obj: Object = self.into();
		return Ok(obj.into());
	}
}

impl Message
{
	pub const ResourceName: &'static str = "message";
}
