use anyhow::Result;

use std::fs::{read_to_string, File};
use std::io;
use std::io::Write;
use std::process::exit;

use json::JsonValue;

use crate::config::get_config_path;
use crate::storage::{Store, StoreValue};

pub fn init_store(force_create: bool) -> JsonValue {
	let path = get_config_path();

	if !path.exists() && force_create {
		let mut file = File::create(get_config_path()).unwrap();
		write!(file, "{}", "{}").unwrap();
	} else if !path.exists() {
		eprintln!("config file does not exist at '{:?}'", &path);
		exit(1);
	}

	let json = json::parse(&read_to_string(&path).unwrap()).unwrap();

	if !json.is_object() {
		eprintln!("config file is not a JSON file ('{:?}')", &path);
		exit(1);
	}

	json
}

#[derive(Clone, Debug)]
pub struct JSONStore {
	store: JsonValue,
}

impl JSONStore {
	pub fn new() -> Self {
		return Self {
			store: init_store(false),
		};
	}

	pub fn with_force_create(force_create: bool) -> Self {
		return Self {
			store: init_store(force_create),
		};
	}

	fn save_store(&mut self) -> Result<(), io::Error> {
		let mut file = File::create(get_config_path())?;

		let json_string = json::stringify_pretty(self.store.clone(), 2);

		write!(file, "{}", json_string)?;

		Ok(())
	}
}

impl Store for JSONStore {
	fn all(&self) -> Result<Vec<(String, StoreValue)>> {
		Ok(
			self
				.store
				.entries()
				.map(|(key, value)| (key.to_owned(), value.into()))
				.collect(),
		)
	}

	fn get(&self, key: &str) -> Result<Option<StoreValue>> {
		if !self.store.has_key(key) {
			return Ok(None);
		}

		Ok(Some(self.store[key].clone().into()))
	}

	fn set(&mut self, key: &str, value: StoreValue) -> Result<StoreValue> {
		self.store.insert(key, value.clone()).unwrap();

		self.save_store().unwrap();

		Ok(value)
	}

	fn remove(&mut self, key: &str) -> Result<Option<StoreValue>> {
		if !self.store.has_key(key) {
			return Ok(None);
		}

		let value = self.store.remove(key);

		self.save_store().unwrap();

		return Ok(Some(value.into()));
	}

	fn clear(&mut self) -> Result<usize> {
		let len = self.store.len();
		self.store.clear();

		self.save_store().unwrap();

		return Ok(len);
	}
}
