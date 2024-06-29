mod config;

#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use ::anyhow::Result;
use ::directories::ProjectDirs;
#[cfg(not(test))]
use ::toml;
pub use self::config::Config;

#[cfg(not(test))]
pub const ConfigPath: &'static str = "./config.toml";

const ProjectQualifier: &'static str = "";
const ProjectOrganization: &'static str = "";
const ProjectApplication: &'static str = "VttServer";

#[cfg(not(test))]
pub fn loadConfig() -> Result<Config>
{
	let config = readConfig(ConfigPath)?;
	return Ok(config);
}

#[cfg(not(test))]
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
