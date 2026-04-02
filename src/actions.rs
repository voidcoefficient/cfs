use std::fs::File;
use std::io::Write;
use std::path::Path;

use json::JsonValue;
use seahorse::{ActionError, ActionResult, Context};

use crate::config::get_config_path;
use crate::error::invalid;
use crate::json_object::{get_json_object_or_create, set_json_object};

pub fn init_action(c: &Context) -> ActionResult {
	let config_path = get_config_path();
	let path = Path::new(&config_path);

	if path.exists() {
		println!("config file already exists");
	} else {
		clear_action(c)?;
	}

	Ok(())
}

pub fn list_action(c: &Context) -> ActionResult {
	let conf = get_json_object_or_create(c.bool_flag("force-create"));

	for (key, value) in conf.entries() {
		println!("{}\t{}", key, value);
	}

	Ok(())
}

pub fn clear_action(_c: &Context) -> ActionResult {
	let mut file = File::create(get_config_path()).unwrap();

	write!(file, "{}", "{}").expect("couldn't overwrite config file");
	println!("cleared config file at '{:?}'", get_config_path());

	Ok(())
}

pub fn get_action(c: &Context) -> ActionResult {
	if c.args.len() != 1 {
		return Err(invalid("command"));
	}

	let conf = get_json_object_or_create(c.bool_flag("force-create"));
	let key = c.args.get(0);

	let Some(key) = key else {
		return Err(invalid("key"));
	};

	if conf.has_key(&key) {
		println!("{}", conf[key]);
		return Ok(());
	}

	if c.bool_flag("ignore-null") {
		println!();
	} else {
		return Err(ActionError {
			message: format!("could not find key '{}'", key),
		});
	}

	Ok(())
}

pub fn set_action(c: &Context) -> ActionResult {
	if c.args.len() != 2 {
		return Err(invalid("command"));
	}

	let mut conf = get_json_object_or_create(c.bool_flag("force-create"));

	let Some(key) = c.args.get(0) else {
		return Err(invalid("key"));
	};

	let Some(value_str) = c.args.get(1) else {
		return Err(invalid("value"));
	};

	let json_value = JsonValue::from(value_str.as_str());
	let value = json_value.as_str().unwrap();

	if conf.has_key(key) {
		conf.remove(key);
	}

	conf.insert(key, value).unwrap();

	match set_json_object(conf) {
		Ok(_) => println!("'{}' -> '{}'", key, value),
		Err(err) => eprintln!("{}", err),
	}

	Ok(())
}

pub fn remove_action(c: &Context) -> ActionResult {
	let mut conf = get_json_object_or_create(c.bool_flag("force-create"));
	let Some(key) = c.args.get(0) else {
		return Err(invalid("key"));
	};

	if !conf.has_key(&key) {
		println!("key '{}' was not found", key);
		return Ok(());
	}

	conf.remove(&key);

	match set_json_object(conf) {
		Ok(_) => println!("updated config file"),
		Err(err) => eprintln!("{}", err),
	}

	Ok(())
}
