pub mod json;
mod sqlite;
mod value;

pub use value::StoreValue;

pub trait Store {
	fn all(&self) -> Vec<(String, StoreValue)>;

	fn get(&self, key: &str) -> Option<StoreValue>;
	fn set(&mut self, key: &str, value: StoreValue) -> StoreValue;
	fn remove(&mut self, key: &str) -> Option<StoreValue>;

	fn clear(&mut self);
}
