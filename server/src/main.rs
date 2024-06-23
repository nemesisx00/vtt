mod config;
mod data;
mod net;
mod util;

use std::path::PathBuf;
use ::anyhow::Result;
use ::flexi_logger::{Duplicate, FileSpec, Logger};
use ::log::{error, info};
use ::tokio::main;
use ::tokio::signal;
use ::tokio_util::task::TaskTracker;
use ::tokio_util::sync::CancellationToken;
use crate::config::{loadConfig, localDataPath};
use crate::data::getDatabase;
use crate::net::WebSocketServer;

const LoggerLevel: &'static str = "info";
const LoggerDir: &'static str = "log";
const LoggerDefaultBaseDir: &'static str = ".";

#[main(flavor = "current_thread")]
async fn main() -> Result<()>
{
	let config = loadConfig()?;
	
	{
		let mut db = getDatabase().lock().await;
		db.initialize(config.database.clone()).await?;
	}
	
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
	
	let cancelToken = CancellationToken::new();
	let tracker = TaskTracker::new();
	
	let serverToken = cancelToken.clone();
	tracker.spawn(async move {
		let server = WebSocketServer::from(config);
		let _ = server.start(serverToken).await;
	});
	tracker.close();
	
	//Graceful shut down
	match signal::ctrl_c().await
	{
		Ok(()) => cancelToken.cancel(),
		Err(e) => error!("Error waiting for Ctrl+C: {:?}", e),
	}
	
	tracker.wait().await;
	info!("Server stopped");
	return Ok(());
}
