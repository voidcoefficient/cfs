use std::path::Path;

use seahorse::{ActionError, ActionResult, Context};

use crate::config::get_db_path;
use crate::error::{invalid, to_action_error};
use crate::storage::sqlite::SQLiteStore;
use crate::storage::{self, Store, StoreValue};

pub fn init_action(_c: &Context) -> ActionResult {
	let config_path = get_db_path();
	let path = Path::new(&config_path);

	if path.exists() {
		println!("config file already exists");
	}

	SQLiteStore::from_path(path);

	Ok(())
}

pub fn list_action(_c: &Context) -> ActionResult {
	let store = storage::load_storage();

	for (key, value) in store.all().map_err(to_action_error)?.iter() {
		println!("{}\t{}", key, value);
	}

	Ok(())
}

pub fn clear_action(_c: &Context) -> ActionResult {
	let mut store = storage::load_storage();

	let count = store.clear().map_err(to_action_error)?;

	println!("removed {} keys from store", count);

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

	let store = storage::load_storage();

	let value = store.get(key).map_err(to_action_error)?;

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

	let mut store = storage::load_storage();

	let value = StoreValue::Value(value_str.to_owned());
	store.set(key, value.clone()).map_err(to_action_error)?;

	println!("'{}' -> '{}'", key, value);

	Ok(())
}

pub fn remove_action(c: &Context) -> ActionResult {
	let Some(key) = c.args.get(0) else {
		return Err(invalid("key"));
	};

	let mut store = storage::load_storage();

	match store.remove(key).map_err(to_action_error)? {
		Some(value) => println!("{}\t{}", key, value),
		None => {
			println!("key '{}' was not found", key);
		}
	}

	Ok(())
}
