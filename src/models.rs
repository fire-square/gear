use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
pub struct User {
	pub id: Uuid,
	pub username: String,
}
