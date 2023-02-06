use anyhow::Result;
use axum::Json;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env::var;
use std::string::String;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UuidTakeResponse {
	id: Uuid,
	hash: [u8; 32],
}

fn get_hash_secret() -> String {
	var("HASH_SECRET").unwrap_or_else(|_| String::from("secret"))
}

pub async fn take_uuid() -> Result<Json<UuidTakeResponse>, crate::errors::AppError> {
	let id = Uuid::new_v4();

	let mut hasher = Sha256::new();
	hasher.update(id);
	hasher.update(get_hash_secret());
	let hash = hasher.finalize();

	let response = UuidTakeResponse {
		id,
		hash: hash.into(),
	};
	Ok(Json(response))
}
