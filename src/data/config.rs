pub struct Config {
	pub version: i64,
}

impl super::DataConnection {
	pub fn set_config(&self, version: i64) -> Result<(), rusqlite::Error> {
		let rows = self
			.db
			.execute("update Config set version = ?", &[version])?;
		if rows <= 0 {
			self.new_config()?;
		}
		Ok(())
	}

	fn new_config(&self) -> Result<Config, rusqlite::Error> {
		let config = Config {
			version: super::sql::DATABASE_VERSION,
		};
		self.db
			.execute("insert into Config (version) values (?)", &[config.version])?;
		Ok(config)
	}

	pub fn get_config(&self) -> Result<Config, rusqlite::Error> {
		Ok(self.db.prepare("select version from Config")?.query_row(
			rusqlite::params![],
			|row| {
				Ok(Config {
					version: row.get(0)?,
				})
			},
		)?)
	}
}
