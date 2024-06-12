use ::serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, PartialEq, Eq, Serialize_repr)]
#[repr(i32)]
pub enum DatabaseType
{
	Memory,
	RocksDB,
}

impl Default for DatabaseType
{
	fn default() -> Self
	{
		return Self::Memory;
	}
}
