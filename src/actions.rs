use std::path::Path;

use seahorse::{ActionError, ActionResult, Context};

use crate::config::get_config_path;
use crate::error::invalid;
use crate::storage::json::CfsJSONStore;
use crate::storage::{CfsStorage, CfsValue};

pub fn init_action(_c: &Context) -> ActionResult {
	let config_path = get_config_path();
	let path = Path::new(&config_path);

	if path.exists() {
		println!("config file already exists");
	}

	CfsJSONStore::with_force_create(true);

	Ok(())
}

pub fn list_action(c: &Context) -> ActionResult {
	let store = CfsJSONStore::with_force_create(c.bool_flag("force-create"));

	for (key, value) in store.all().iter() {
		println!("{}\t{}", key, value);
	}

	Ok(())
}

pub fn clear_action(_c: &Context) -> ActionResult {
	let mut store = CfsJSONStore::new();

	store.clear();

	println!("cleared config file at '{:?}'", get_config_path());

	Ok(())
}

pub fn get_action(c: &Context) -> ActionResult {
	if c.args.len() != 1 {
		return Err(invalid("command"));
	}

	let key = c.args.get(0).to_owned();

	let Some(key) = key else {
		return Err(invalid("key"));
	};

	let store = CfsJSONStore::new();

	let value = store.get(key);

	match value {
		Some(v) => {
			println!("{}", v)
		}
		None => {
			if c.bool_flag("ignore_null") {
				println!();
			} else {
				return Err(ActionError {
					message: format!("could not find key '{}'", key),
				});
			}
		}
	}

	Ok(())
}

pub fn set_action(c: &Context) -> ActionResult {
	if c.args.len() != 2 {
		return Err(invalid("command"));
	}

	let Some(key) = c.args.get(0) else {
		return Err(invalid("key"));
	};

	let Some(value_str) = c.args.get(1) else {
		return Err(invalid("value"));
	};

	let mut store = CfsJSONStore::new();

	let value = CfsValue::Value(value_str.to_owned());
	store.set(key, value.clone());

	println!("{}\t{}", key, value);

	Ok(())
}

pub fn remove_action(c: &Context) -> ActionResult {
	let Some(key) = c.args.get(0) else {
		return Err(invalid("key"));
	};

	let mut store = CfsJSONStore::new();

	match store.remove(key) {
		Some(value) => println!("{}\t{}", key, value),
		None => {
			println!("key '{}' was not found", key);
		}
	}

	Ok(())
}
