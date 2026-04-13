use anyhow::Context;
use dirs::{config_dir, data_local_dir};
use std::{fs, path::PathBuf};

use serde::Deserialize;

pub fn get_config_path() -> PathBuf {
	return config_dir()
		.expect("couldn't find config directory")
		.join("cfs/");
}

pub fn get_default_store_path() -> String {
	return data_local_dir()
		.expect("couldn't find data directory")
		.join("cfs.db")
		.to_str()
		.unwrap()
		.to_owned();
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum StoreType {
	Sqlite,
}
impl Default for StoreType {
	fn default() -> Self {
		Self::Sqlite
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	#[serde(default)]
	pub store: StoreType,
	#[serde(default = "get_default_store_path")]
	pub store_path: String,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			store: StoreType::Sqlite,
			store_path: get_default_store_path(),
		}
	}
}

impl Config {
	pub fn load() -> anyhow::Result<Self> {
		let config_file_path = get_config_path().join("config.kdl");

		if !config_file_path.exists() {
			return Ok(Self::default());
		}

		let s = fs::read_to_string(config_file_path).context("couldn't read config file")?;
		let mut config = serde_kdl2::from_str::<Self>(&s).context("couldn't parse config file")?;

		let path = shellexpand::full(&config.store_path)
			.context("failed to parse store path")?
			.to_string();

		config.store_path = path;

		Ok(config)
	}
}
