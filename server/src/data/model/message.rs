use ::chrono::NaiveDateTime;
use ::diesel::{Insertable, Selectable, Queryable};
use super::super::schema;

pub const CreateTable_Messages: &'static str = r#"CREATE TABLE IF NOT EXISTS messages
(
	id INTEGER PRIMARY KEY,
	text TEXT NOT NULL,
	timestamp TIMESTAMP NOT NULL,
	userId INTEGER
)"#;

pub const DropTable_Messages: &'static str = "DROP TABLE messages";

#[derive(Clone, Debug, Default, PartialEq, Selectable, Queryable)]
#[diesel(table_name = schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message
{
	pub id: i32,
	pub text: String,
	pub timestamp: NaiveDateTime,
	pub userId: Option<i32>,
}

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = schema::messages)]
pub struct NewMessage
{
	pub text: String,
	pub timestamp: NaiveDateTime,
	pub userId: Option<i32>,
}
