use std::path::PathBuf;

mod apartment;
mod config;
mod sql;

pub struct DataConnection {
	db: rusqlite::Connection,
}

impl DataConnection {
	pub fn new(path: &PathBuf) -> Result<Self, rusqlite::Error> {
		match std::fs::create_dir_all(path.parent().unwrap_or_else(|| path)) {
			Err(err) => log::error!("failed to create config directory: {}", err),
			_ => {}
		}

		log::info!(
			"database is saved in: {}",
			path.to_str().unwrap_or_else(|| "(invalid path)")
		);
		let db = Self {
			db: rusqlite::Connection::open(path)?,
		};

		db.ensure_created()?;
		Ok(db)
	}

	pub fn default_path() -> Result<Self, rusqlite::Error> {
		DataConnection::new(
			&match dirs::config_dir() {
				Some(config_dir) => config_dir.join("gfr"),
				None => PathBuf::new(),
			}
			.join("data.gfr"),
		)
	}

	fn ensure_created(&self) -> Result<(), rusqlite::Error> {
		// Check if config exists
		let updated = match self.get_config() {
			Ok(config) => (config.version <= sql::DATABASE_VERSION),
			Err(_) => false,
		};
		if !updated {
			let rows = self.db.execute(sql::MIGRATION, rusqlite::params![])?;
			self.set_config(sql::DATABASE_VERSION)?;
			log::info!(
				"database updated to version {} (affected rows: {})",
				sql::DATABASE_VERSION,
				rows
			);
		}

		Ok(())
	}
}
