mod apartment;
mod config;
mod sql;

pub struct DataConnection {
	db: rusqlite::Connection,
}

impl DataConnection {
	pub fn new(database_url: &str) -> Result<Self, rusqlite::Error> {
		let db = Self {
			db: rusqlite::Connection::open(database_url)?,
		};

		db.ensure_created()?;
		Ok(db)
	}

	fn ensure_created(&self) -> Result<(), rusqlite::Error> {
		// Check if config exists
		let updated = match self.get_config() {
			Ok(config) => (config.version >= sql::DATABASE_VERSION),
			Err(_) => false,
		};
		if !updated {
			let rows = self.db.execute(sql::MIGRATION, rusqlite::params![])?;
			println!(
				"database updated to version {}: {} rows affected",
				sql::DATABASE_VERSION,
				rows
			);
		}

		// If there's no known version, set it
		match self.get_config() {
			Ok(_) => (),
			Err(_) => {
				self.new_config()?;
			}
		}

		Ok(())
	}
}
