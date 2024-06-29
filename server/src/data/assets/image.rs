use std::fs::read;
use std::path::PathBuf;
use ::anyhow::Result;
use super::Asset;

pub struct Image
{
	/// Absolute path to the image file.
	path: PathBuf,
	data: Vec<u8>,
}

impl Asset for Image
{
	fn bytes(&self) -> Result<Vec<u8>>
	{
		return Ok(self.data.to_owned());
	}
	
	fn load(path: PathBuf) -> Result<Self>
		where Self: Sized
	{
		let bytes = read(path.clone())?;
		
		return Ok(Self
		{
			path,
			data: bytes,
		});
	}
}
