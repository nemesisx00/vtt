use ::diesel::{Insertable, Selectable, Queryable};
use super::super::schema;

pub const CreateTable_ImageAssets: &'static str = r#"CREATE TABLE IF NOT EXISTS imageAssets
(
	id INTEGER PRIMARY KEY,
	height BIGINT NOT NULL,
	path TEXT NOT NULL,
	width BIGINT NOT NULL
)"#;

pub const DropTable_ImageAssets: &'static str = "DROP TABLE imageAssets";

#[derive(Clone, Debug, Default, PartialEq, Selectable, Queryable)]
#[diesel(table_name = schema::imageAssets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ImageAsset
{
	pub id: i32,
	pub height: i64,
	/// Path to image file, relative to the assets directory.
	pub path: String,
	pub width: i64,
}

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = schema::imageAssets)]
pub struct NewImageAsset
{
	pub height: i64,
	/// Path to image file, relative to the assets directory.
	pub path: String,
	pub width: i64,
}
