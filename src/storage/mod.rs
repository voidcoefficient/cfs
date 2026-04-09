use anyhow::Result;

pub mod sqlite;
mod value;

pub use value::StoreValue;

use crate::config::{Config, StoreType};

pub trait Store {
	fn all(&self) -> Result<Vec<(String, StoreValue)>>;

	fn get(&self, key: &str) -> Result<Option<StoreValue>>;
	fn set(&mut self, key: &str, value: StoreValue) -> Result<StoreValue>;
	fn remove(&mut self, key: &str) -> Result<Option<StoreValue>>;

	fn clear(&mut self) -> Result<usize>;
}

pub fn load_storage(config: &Config) -> impl Store {
	match config.store {
		StoreType::Sqlite => return sqlite::SQLiteStore::new(&config.store_path),
	}
}
