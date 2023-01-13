use anyhow::Result;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::string::String;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct UuidClaim {
	id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UuidTakeResponse {
	id: Uuid,
	token: String,
}

fn get_jwt_secret() -> String {
	var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"))
}

fn create_uuid_token(id: &Uuid) -> Result<String> {
	let claim = UuidClaim { id: id.clone() };
	Ok(encode(
		&Header::default(),
		&claim,
		&EncodingKey::from_secret(get_jwt_secret().as_ref()),
	)?)
}

pub async fn take_uuid() -> Result<Json<UuidTakeResponse>, crate::errors::AppError> {
	let id = Uuid::new_v4();
	let token = create_uuid_token(&id)?;
	let response = UuidTakeResponse { id, token };
	Ok(Json(response))
}
