use ::anyhow::Result;
use ::fastwebsockets::upgrade::{UpgradeFut, upgrade};
use ::http_body_util::Empty;
use ::hyper::{Request, Response};
use ::hyper::body::{Incoming, Bytes};
use ::hyper::server::conn::http1;
use ::hyper::service::service_fn;
use ::hyper_util::rt::TokioIo;
use ::log::{error, info};
use ::tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use crate::config::Config;
use super::client::WebSocketClient;

#[derive(Clone, Default)]
pub struct WebSocketServer
{
	config: Config,
}

impl From<Config> for WebSocketServer
{
	fn from(value: Config) -> Self
	{
		return Self
		{
			config: value.to_owned(),
			..Default::default()
		};
	}
}

impl WebSocketServer
{
	pub async fn start(&self, token: CancellationToken) -> Result<()>
	{
		let listener = TcpListener::bind(self.config.network.fullAddress()).await?;
		info!("Listening on {}", self.config.network.fullAddress());
		
		loop
		{
			let cancelToken = token.clone();
			tokio::select! {
				_ = token.cancelled() => break,
				
				result = listener.accept() => {
					if let Ok((stream, _)) = result
					{
						info!("Client connected");
						let conf = self.config.clone();
						tokio::spawn(async move {
							let io = TokioIo::new(stream);
							let connectionFuture = http1::Builder::new()
								.serve_connection(io, service_fn(|request: Request<Incoming>| async {
									let ct = cancelToken.clone();
									return serverUpgrade(request, ct, conf.clone()).await;
								}))
								.with_upgrades();
							
							if let Err(e) = connectionFuture.await
							{
								error!("An error occurred: {:?}", e);
								cancelToken.cancel();
							}
						});
					}
				}
			}
		}
		
		return Ok(());
	}
}

async fn handleClient(future: UpgradeFut, token: CancellationToken, config: Config) -> Result<()>
{
	let mut client = WebSocketClient::fromUpgradeFut(future, config).await?;
	client.start(token).await?;
	return Ok(());
}

async fn serverUpgrade(mut request: Request<Incoming>, token: CancellationToken, config: Config) -> Result<Response<Empty<Bytes>>>
{
	let (response, future) = upgrade(&mut request)?;
	
	let cancelToken = token.clone();
	tokio::task::spawn(async move {
		if let Err(e) = tokio::task::unconstrained(handleClient(future, cancelToken, config)).await
		{
			error!("Error in websocket connection: {}", e);
		}
	});
	
	return Ok(response);
}
