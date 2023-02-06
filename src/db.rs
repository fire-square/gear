use crate::models::User;
use crate::schema::users::dsl::*;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use tokio::sync::Mutex;
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Db {
	connection: Mutex<PgConnection>,
}

impl Db {
	pub fn new() -> Result<Self> {
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
		let mut conn = PgConnection::establish(&database_url)?;
		conn.run_pending_migrations(MIGRATIONS).unwrap();

		let db = Self {
			connection: Mutex::new(conn),
		};

		Ok(db)
	}

	pub async fn create_user(&self, name: String) -> Result<Uuid> {
		let new_uuid = Uuid::new_v4();
		let new_user = User {
			id: new_uuid,
			username: name,
		};

		diesel::insert_into(users)
			.values(&new_user)
			.execute(&mut *self.connection.lock().await)?;
		Ok(new_uuid)
	}
}
