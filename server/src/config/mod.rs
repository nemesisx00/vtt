mod config;

use std::fs;
use ::anyhow::Result;
use ::directories::ProjectDirs;
use ::toml;
pub use self::config::{Config, ConfigDatabase};

pub const ConfigPath: &'static str = "./config.toml";

const ProjectQualifier: &'static str = "";
const ProjectOrganization: &'static str = "";
const ProjectApplication: &'static str = "VttServer";

pub fn loadConfig() -> Result<Config>
{
	let config = readConfig(ConfigPath)?;
	return Ok(config);
}

pub fn readConfig(path: &str) -> Result<Config>
{
	let text = fs::read_to_string(path)?;
	let config = toml::from_str::<Config>(text.as_str())?;
	return Ok(config);
}

pub fn localDataPath() -> Option<String>
{
	let dirs = projectDirs()?;
	let path = dirs.data_local_dir();
	return Some(path.to_str()?.to_string());
}

fn projectDirs() -> Option<ProjectDirs>
{
	return ProjectDirs::from(
		ProjectQualifier,
		ProjectOrganization,
		ProjectApplication
	);
}