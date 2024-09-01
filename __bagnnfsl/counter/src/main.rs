use std::{env::args, error::Error, sync::Arc};

use axum::{
	extract::State,
	http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderMap, HeaderValue, Method, StatusCode},
	response::IntoResponse,
	routing::{get, put},
};
use tokio::{
	fs,
	io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
	net::TcpListener,
	sync::Mutex,
};
use tower_http::cors::CorsLayer;

struct Files {
	counter: fs::File,
	ips: fs::File,
}

impl Files {
	async fn new() -> Self {
		let counter_file = fs::File::options()
			.write(true)
			.read(true)
			.create(true)
			.append(false)
			.open("./counter.txt")
			.await
			.expect("can't open a handle to ./counter.txt!");
		let ips_file = fs::File::options()
			.write(true)
			.read(true)
			.create(true)
			.append(false)
			.open("./ips.txt")
			.await
			.expect("can't open a handle to ./ips.txt!");
		Self { counter: counter_file, ips: ips_file }
	}

	/// returns if ip was seen before
	async fn try_ip(&mut self, ip: &str) -> Result<bool, Box<dyn Error>> {
		self.ips.seek(std::io::SeekFrom::Start(0)).await?;
		let mut buf = String::new();
		let _ = self.ips.read_to_string(&mut buf).await?;

		let found = buf.lines().any(|x| x == ip);
		if !found {
			let _ = self.ips.write_all(ip.as_bytes()).await;
			let _ = self.ips.write(b"\n").await;
			self.ips.flush().await.unwrap();
		}
		self.ips.seek(std::io::SeekFrom::Start(0)).await?;
		Ok(found)
	}

	async fn increment(&mut self) -> Result<(), Box<dyn Error>> {
		let current_count = self.current_count().await?;
		self.counter.write_all((current_count + 1).to_string().as_bytes()).await?;
		Result::<(), Box<dyn Error>>::Ok(())
	}

	async fn current_count(&mut self) -> Result<usize, Box<dyn Error>> {
		self.counter.seek(std::io::SeekFrom::Start(0)).await?;
		let mut buf = String::new();
		let _ = self.counter.read_to_string(&mut buf).await?;
		self.counter.seek(std::io::SeekFrom::Start(0)).await?;
		let current_count = buf.trim().parse()?;
		Ok(current_count)
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let files = Files::new().await;

	let app = axum::Router::new()
		.route("/", put(increment))
		.route("/", get(counter))
		.layer(
			CorsLayer::new()
				.allow_methods([Method::GET, Method::PUT])
				.allow_origin("https://buddhistmemes.com".parse::<HeaderValue>().unwrap()),
		)
		.with_state(Arc::new(Mutex::new(files)));

	axum::serve(
		TcpListener::bind((
			"0.0.0.0",
			args()
				.nth(1)
				.expect("provide the port as cli argument")
				.parse()
				.expect("port is not a valid number"),
		))
		.await
		.expect("failed to build TcpListener"),
		app,
	)
	.await
	.unwrap();
}

async fn increment(State(files): State<Arc<Mutex<Files>>>, headers: HeaderMap) -> StatusCode {
	let mut files = files.lock().await;

	let ip = headers.get("x-forwarded-for").or(headers.get("cf-connecting-ip"));
	if let Some(ip) = ip {
		match files.try_ip(ip.to_str().unwrap()).await {
			Ok(true) => {
				println!("already incremented: {:?}", ip);
				return StatusCode::NO_CONTENT;
			}
			Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
			_ => {}
		}
	}

	match files.increment().await {
		Ok(()) => StatusCode::OK,
		Err(err) => {
			println!("Error in /increment:\n{}", err);
			StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}

async fn counter(State(files): State<Arc<Mutex<Files>>>) -> Result<impl IntoResponse, StatusCode> {
	match files.lock().await.current_count().await {
		Ok(x) => {
			let mut headers = HeaderMap::new();
			headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
			Ok((headers, x.to_string()))
		}
		Err(err) => {
			println!("Error in /counter:\n{}", err);
			Err(StatusCode::INTERNAL_SERVER_ERROR)
		}
	}
}
