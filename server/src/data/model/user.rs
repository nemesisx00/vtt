use ::diesel::{Insertable, Selectable, Queryable};
use super::super::schema;

pub const CreateTable_Users: &'static str = r#"CREATE TABLE IF NOT EXISTS users
(
	id INTEGER PRIMARY KEY,
	label TEXT DEFAULT NULL,
	name TEXT NOT NULL
)"#;

pub const DropTable_Users: &'static str = "DROP TABLE users";

#[derive(Clone, Debug, Default, PartialEq, Selectable, Queryable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User
{
	pub id: i32,
	pub label: Option<String>,
	pub name: String,
}

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser
{
	pub label: Option<String>,
	pub name: String,
}
