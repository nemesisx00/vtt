#![allow(non_snake_case, non_upper_case_globals)]

use ::anyhow::Result;
use ::fastwebsockets::{FragmentCollector, OpCode};
use ::fastwebsockets::upgrade::{upgrade, UpgradeFut};
use ::http_body_util::Empty;
use ::hyper::{Request, Response};
use ::hyper::body::{Incoming, Bytes};
use ::hyper::server::conn::http1;
use ::hyper::service::service_fn;
use ::hyper_util::rt::TokioIo;
use ::tokio::main;
use ::tokio::net::TcpListener;

const BindAddress: &'static str = "127.0.0.1:7890";

#[main(flavor = "current_thread")]
async fn main() -> Result<()>
{
	let listener = TcpListener::bind(BindAddress).await?;
	println!("Listening on {}", BindAddress);
	loop
	{
		let (stream, _) = listener.accept().await?;
		println!("Client connected");
		
		tokio::spawn(async move {
			let io = TokioIo::new(stream);
			let connectionFuture = http1::Builder::new()
				.serve_connection(io, service_fn(serverUpgrade))
				.with_upgrades();
			
			if let Err(e) = connectionFuture.await
			{
				println!("An error occurred: {:?}", e);
			}
		});
	}
}

async fn handleClient(future: UpgradeFut) -> Result<()>
{
	let mut ws = FragmentCollector::new(future.await?);
	
	loop
	{
		let frame = ws.read_frame().await?;
		match frame.opcode
		{
			OpCode::Close => {
				println!("Client disconnected!");
				break;
			},
			//Echo back
			OpCode::Text | OpCode::Binary => {
				if frame.opcode == OpCode::Text
				{
					match std::str::from_utf8(frame.payload.as_ref())
					{
						Ok(s) => println!("Input: '{}'", s),
						Err(e) => println!("Failed to parse packet: {:?}", e),
					}
				}
				
				ws.write_frame(frame).await?;
			},
			_ => {},
		}
	}
	
	return Ok(());
}

async fn serverUpgrade(mut request: Request<Incoming>) -> Result<Response<Empty<Bytes>>>
{
	let (response, future) = upgrade(&mut request)?;
	
	tokio::task::spawn(async move {
		if let Err(e) = tokio::task::unconstrained(handleClient(future)).await
		{
			eprintln!("Error in websocket connection: {}", e);
		}
	});
	
	return Ok(response);
}
