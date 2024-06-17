use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Thing};
use super::Message;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	pub id: Option<Thing>,
	pub label: Option<String>,
	pub name: String,
	pub messages: Vec<Message>,
}

impl IntoResource<Resource> for User
{
	fn into_resource(self) -> Result<Resource>
	{
		let mut obj = Object::default();
		if let Some(id) = &self.id
		{
			obj.insert("id".into(), id.to_owned().into());
		}
		
		if let Some(label) = &self.label
		{
			obj.insert("label".into(), label.to_owned().into());
		}
		
		obj.insert("name".into(), self.name.to_owned().into());
		
		return Ok(obj.into());
	}
}

impl User
{
	pub const ResourceName: &'static str = "user";
}
