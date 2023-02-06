use anyhow::Result;
use axum::extract::Path;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Router};
use diesel::PgConnection;
use log::info;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

mod db;
mod errors;
mod idtake;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
	log_panics::init();

	let conn = Arc::new(Mutex::new(db::establish_connection()?));

	let app = Router::new()
		.route("/", get(hello))
		.route("/take_uuid", get(idtake::take_uuid))
		.route("/users/:name", post(create_user))
		.layer(Extension(conn));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	info!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await?;

	Ok(())
}

async fn create_user(
	Path(name): Path<String>,
	conn: Extension<Arc<Mutex<PgConnection>>>,
) -> Result<String, errors::AppError> {
	Ok(db::create_user(&mut conn.lock().unwrap(), name)?.to_string())
}

async fn hello() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}
