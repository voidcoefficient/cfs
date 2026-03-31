pub mod json;
mod value;

pub use value::CfsValue;

pub trait CfsStorage {
	fn all(&self) -> Vec<(String, CfsValue)>;

	fn get(&self, key: &str) -> Option<CfsValue>;
	fn set(&mut self, key: &str, value: CfsValue) -> CfsValue;
	fn remove(&mut self, key: &str) -> Option<CfsValue>;

	fn clear(&mut self);
}
