use ::diesel::{Insertable, Selectable, Queryable};
use super::super::schema;

pub const CreateTable_Scenes2D: &'static str = r#"CREATE TABLE IF NOT EXISTS scenes2d
(
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	backgroundId INTEGER NOT NULL
)"#;

pub const DropTable_Scenes2D: &'static str = "DROP TABLE scenes2d";

#[derive(Clone, Debug, Default, PartialEq, Selectable, Queryable)]
#[diesel(table_name = schema::scenes2d)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Scene2D
{
	pub id: i32,
	pub name: String,
	pub backgroundId: i32,
}

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = schema::scenes2d)]
pub struct NewScene2D
{
	pub name: String,
	pub backgroundId: i32,
}
