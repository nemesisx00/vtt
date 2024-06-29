mod image;

use std::path::PathBuf;
use ::anyhow::{Error, Result};
use crate::getConfig;
use crate::config::localDataPath;
pub use self::image::Image;

const AssetDirectoryName: &'static str = "assets";

pub trait Asset
{
	fn bytes(&self) -> Result<Vec<u8>>;
	
	fn load(path: PathBuf) -> Result<Self>
	where Self: Sized;
}

pub fn loadAsset<T>(relativePath: String) -> Result<T>
	where T: Asset
{
	return match getAssetDirectory()
	{
		None => Err(Error::msg("Failed to retrieve asset directory path")),
		Some(absolutePath) => T::load(absolutePath.join(relativePath)),
	};
}

fn getAssetDirectory() -> Option<PathBuf>
{
	let config = getConfig();
	
	let path = match &config.assets.path
	{
		None => localDataPath(),
		Some(path) => Some(path.to_owned()),
	};
	
	return match path
	{
		None => None,
		Some(root) => Some(PathBuf::from(root).join(AssetDirectoryName)),
	}
}
