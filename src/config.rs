use std::{env::home_dir, path::PathBuf};

pub fn get_config_path() -> PathBuf {
	let home_folder = home_dir().expect("couldn't find home directory");
	let path = home_folder.join(".cfs.json");

	return path;
}
