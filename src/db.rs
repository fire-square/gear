use crate::models::User;
use crate::schema::users::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let mut conn = PgConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
	conn.run_pending_migrations(MIGRATIONS).unwrap();
	conn
}

pub fn create_user(conn: &mut PgConnection, name: String) -> Result<Uuid, anyhow::Error> {
	let new_uuid = Uuid::new_v4();
	let new_user = User {
		id: new_uuid,
		username: name,
	};

	diesel::insert_into(users).values(&new_user).execute(conn)?;
	Ok(new_uuid)
}
