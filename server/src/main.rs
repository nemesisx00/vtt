mod config;
mod data;
mod net;
mod util;

use std::path::PathBuf;
use std::sync::OnceLock;
use ::anyhow::Result;
use ::flexi_logger::{Duplicate, FileSpec, Logger};
use ::log::{error, info};
use ::tokio::main;
use ::tokio::signal::ctrl_c;
use ::tokio_util::task::TaskTracker;
use ::tokio_util::sync::CancellationToken;
#[cfg(not(test))]
use crate::config::loadConfig;
use crate::config::{localDataPath, Config};
use crate::data::getDatabase;
use crate::net::WebSocketServer;

const LoggerLevel: &'static str = "info";
const LoggerDir: &'static str = "log";
const LoggerDefaultBaseDir: &'static str = ".";

#[cfg(test)]
pub fn getConfig() -> &'static Config
{
	static ConfigLock: OnceLock<Config> = OnceLock::new();
	return ConfigLock.get_or_init(|| Config::getTestConfig());
}

#[cfg(not(test))]
pub fn getConfig() -> &'static Config
{
	static ConfigLock: OnceLock<Config> = OnceLock::new();
	return ConfigLock.get_or_init(|| {
		return match loadConfig()
		{
			Ok(c) => c,
			Err(e) => {
				error!("Error loading configuration file: {:?}", e);
				Config::default()
			}
		};
	});
}

#[main(flavor = "current_thread")]
async fn main() -> Result<()>
{
	initializeDatabase().await;
	initializeLogger()?;
	
	let cancelToken = CancellationToken::new();
	let tracker = TaskTracker::new();
	
	let serverToken = cancelToken.clone();
	tracker.spawn(async move {
		let server = WebSocketServer::default();
		let _ = server.start(serverToken).await;
	});
	tracker.close();
	
	//Graceful shut down
	match ctrl_c().await
	{
		Ok(()) => {
			info!("Graceful shutdown requested via Ctrl+C");
			cancelToken.cancel();
		},
		
		Err(e) => error!("Error waiting for Ctrl+C: {:?}", e),
	}
	
	tracker.wait().await;
	info!("Server stopped");
	return Ok(());
}

async fn initializeDatabase()
{
	let mut db = getDatabase().lock().await;
	match db.initialize().await
	{
		Ok(_) => info!("Database initialized!"),
		Err(e) => error!("Error initializing database: {:?}", e),
	}
}

fn initializeLogger() -> Result<()>
{
	let mut logPath = PathBuf::from(LoggerDefaultBaseDir);
	if let Some(path) = localDataPath()
	{
		logPath = PathBuf::from(path);
	}
	logPath = logPath.join(LoggerDir);
	
	Logger::try_with_str(LoggerLevel)?
		.log_to_file(
			FileSpec::default()
				.directory(logPath)
		)
		.duplicate_to_stderr(Duplicate::Error)
		.duplicate_to_stdout(Duplicate::Info)
		.start()?;
	
	return Ok(());
}
