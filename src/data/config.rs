pub struct Config {
	pub version: i64,
}

impl super::DataConnection {
	pub fn new_config(&self) -> Result<Config, rusqlite::Error> {
		let config = Config {
			version: super::sql::DATABASE_VERSION,
		};
		self.db
			.execute("insert into Config (version) values (?)", &[config.version])?;
		Ok(config)
	}

	pub fn config_exists(&self) -> Result<bool, rusqlite::Error> {
		Ok(self
			.db
			.prepare("select count(name) from sqlite_master where type = ? and name = ?")?
			.query_row(&["table", "Config"], |row| row.get::<usize, i64>(0))?
			> 0)
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
