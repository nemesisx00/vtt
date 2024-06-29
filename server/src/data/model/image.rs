use ::serde::{Deserialize, Serialize};
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::{Object, Value};

const Property_Height: &'static str = "height";
const Property_Path: &'static str = "path";
const Property_Width: &'static str = "width";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImageAsset
{
	pub height: i64,
	/// Path to image file, relative to the assets directory.
	pub path: String,
	pub width: i64,
}

impl Into<Object> for ImageAsset
{
	fn into(self) -> Object
	{
		let mut obj = Object::default();
		
		obj.insert(Property_Height.into(), self.height.into());
		obj.insert(Property_Path.into(), self.path.into());
		obj.insert(Property_Width.into(), self.width.into());
		
		return obj;
	}
}

impl Into<Value> for ImageAsset
{
	fn into(self) -> Value { return Value::Object(self.into()); }
}

impl IntoResource<Resource> for ImageAsset
{
	fn into_resource(self) -> Result<Resource>
	{
		let obj: Object = self.into();
		return Ok(obj.into());
	}
}

impl ImageAsset
{
	pub const ResourceName: &'static str = "imageAsset";
}
