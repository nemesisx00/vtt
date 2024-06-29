use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Thing};
use super::ImageAsset;

const Property_Background: &'static str = "background";
const Property_Id: &'static str = "id";
const Property_Name: &'static str = "name";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scene2D
{
	pub id: Option<Thing>,
	pub name: String,
	pub background: ImageAsset,
}

impl Into<Object> for Scene2D
{
	fn into(self) -> Object
	{
		let mut obj = Object::default();
		
		if let Some(id) = &self.id
		{
			obj.insert(Property_Id.into(), id.to_owned().into());
		}
		
		obj.insert(Property_Name.into(), self.name.into());
		obj.insert(Property_Background.into(), self.background.into());
		
		return obj;
	}
}

impl IntoResource<Resource> for Scene2D
{
	fn into_resource(self) -> Result<Resource>
	{
		let obj: Object = self.into();
		return Ok(obj.into());
	}
}

impl Scene2D
{
	pub const ResourceName: &'static str = "scene2d";
}
