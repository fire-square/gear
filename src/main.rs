use anyhow::Result;
use axum::extract::Path;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Router};
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;

mod db;
mod errors;
mod idtake;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
	log_panics::init();

	let database = Arc::new(db::Db::new()?);

	let app = Router::new()
		.route("/", get(hello))
		.route("/take_uuid", get(idtake::take_uuid))
		.route("/users/:name", post(create_user))
		.layer(Extension(database));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	info!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await?;

	Ok(())
}

async fn create_user(
	Path(name): Path<String>,
	database: Extension<Arc<db::Db>>,
) -> Result<String, errors::AppError> {
	Ok(database.create_user(name).await?.to_string())
}

async fn hello() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}
