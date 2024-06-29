use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Thing};

const Property_Id: &'static str = "id";
const Property_Label: &'static str = "label";
const Property_Name: &'static str = "name";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	pub id: Option<Thing>,
	pub label: Option<String>,
	pub name: String,
}

impl Into<Object> for User
{
	fn into(self) -> Object
	{
		let mut obj = Object::default();
		if let Some(id) = &self.id
		{
			obj.insert(Property_Id.into(), id.to_owned().into());
		}
		
		if let Some(label) = &self.label
		{
			obj.insert(Property_Label.into(), label.to_owned().into());
		}
		
		obj.insert(Property_Name.into(), self.name.to_owned().into());
		
		return obj;
	}
}

impl IntoResource<Resource> for User
{
	fn into_resource(self) -> Result<Resource>
	{
		let obj: Object = self.into();
		return Ok(obj.into());
	}
}

impl User
{
	pub const ResourceName: &'static str = "user";
}
