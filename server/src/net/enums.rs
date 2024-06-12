use ::serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, PartialEq, Eq, Serialize_repr)]
#[repr(i32)]
pub enum Commands
{
	None,
	
	AuthenticateRequest = 100,
	AuthenticateSend,
	AuthenticateFail,
	AuthenticateSuccess,
	
	BroadcastReceive = 200,
	BroadcastSend,
}

impl Default for Commands
{
	fn default() -> Self
	{
		return Self::None;
	}
}
