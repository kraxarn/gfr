mod apartment;
mod config;
mod sql;

pub struct DataConnection {
	path: String,
	options: rocksdb::Options,
}

impl DataConnection {
	pub fn new(database_url: &str) -> Self {
		let mut options = rocksdb::Options::default();
		options.create_if_missing(true);
		options.create_missing_column_families(true);

		Self {
			path: database_url.to_string(),
			options,
		}
	}

	fn open_default(&self) -> Result<rocksdb::DB, rocksdb::Error> {
		rocksdb::DB::open_default(&self.path)
	}

	pub fn open(&self, name: &str) -> Result<rocksdb::DB, rocksdb::Error> {
		rocksdb::DB::open_cf(&self.options, &self.path, vec![name])
	}
}
