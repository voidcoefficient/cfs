use std::{env::home_dir, path::PathBuf};

pub fn get_config_folder_path() -> PathBuf {
	return home_dir().expect("couldn't find home directory");
}

pub fn get_config_path() -> PathBuf {
	return get_config_folder_path().join(".cfs.json");
}

pub fn get_db_path() -> PathBuf {
	return get_config_folder_path().join(".cfs.db");
}
