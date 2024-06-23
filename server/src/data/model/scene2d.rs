use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Thing};
use super::ImageAsset;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scene2D
{
	pub id: Option<Thing>,
	pub name: String,
	pub background: ImageAsset,
}

impl IntoResource<Resource> for Scene2D
{
	fn into_resource(self) -> Result<Resource>
	{
		let mut obj = Object::default();
		
		if let Some(id) = &self.id
		{
			obj.insert("id".into(), id.to_owned().into());
		}
		
		obj.insert("name".into(), self.name.into());
		obj.insert("background".into(), self.background.into());
		
		return Ok(obj.into());
	}
}

impl Scene2D
{
	pub const ResourceName: &'static str = "scene2d";
}
