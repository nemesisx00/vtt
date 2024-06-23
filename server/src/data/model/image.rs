use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Value;
use ::surrealdb::Result;
use ::surrealdb::opt::{IntoResource, Resource};
use ::surrealdb::sql::Object;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImageAsset
{
	pub height: i64,
	pub path: String,
	pub width: i64,
}

impl Into<Value> for ImageAsset
{
	fn into(self) -> Value
	{
		let mut obj = Object::default();
		
		obj.insert("height".into(), self.height.into());
		obj.insert("path".into(), self.path.into());
		obj.insert("width".into(), self.width.into());
		
		return Value::Object(obj);
	}
}

impl IntoResource<Resource> for ImageAsset
{
	fn into_resource(self) -> Result<Resource>
	{
		let mut obj = Object::default();
		
		obj.insert("height".into(), self.height.into());
		obj.insert("path".into(), self.path.into());
		obj.insert("width".into(), self.width.into());
		
		return Ok(obj.into());
	}
}
