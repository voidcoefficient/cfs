use anyhow::Result;

pub mod json;
pub mod sqlite;
mod value;

pub use value::StoreValue;

use crate::config::get_db_path;

pub trait Store {
	fn all(&self) -> Result<Vec<(String, StoreValue)>>;

	fn get(&self, key: &str) -> Result<Option<StoreValue>>;
	fn set(&mut self, key: &str, value: StoreValue) -> Result<StoreValue>;
	fn remove(&mut self, key: &str) -> Result<Option<StoreValue>>;

	fn clear(&mut self) -> Result<usize>;
}

//TODO: Change STORE Based on config.
pub fn load_storage() -> impl Store {
	return sqlite::SQLiteStore::from_path(get_db_path());
}
