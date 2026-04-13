use anyhow::{anyhow, Result};

use rusqlite::OptionalExtension as _;

use crate::storage::{Store, StoreValue};

#[derive(Debug)]
pub struct SQLiteStore {
	connection: rusqlite::Connection,
}

impl SQLiteStore {
	pub fn new(path: &str) -> Self {
		let conn = rusqlite::Connection::open(path).expect("To Open SQLite DB");

		conn
			.execute_batch(include_str!("schema.sql"))
			.expect("To Create DB");

		Self { connection: conn }
	}
}

impl Store for SQLiteStore {
	fn all(&self) -> Result<Vec<(String, super::StoreValue)>> {
		let mut query = self.connection.prepare("SELECT key,value from KV")?;

		let values = query
			.query_map([], |row| Ok((row.get(0)?, StoreValue::Value(row.get(1)?))))?
			.collect::<Result<Vec<_>, _>>()?;

		Ok(values)
	}

	fn get(&self, key: &str) -> Result<Option<super::StoreValue>> {
		let query = self
			.connection
			.query_row(
				"SELECT key,value from KV where key = ?1 LIMIT 1",
				[key],
				|row| Ok(StoreValue::Value(row.get(1)?)),
			)
			.optional()?;

		Ok(query)
	}

	fn set(&mut self, key: &str, value: StoreValue) -> Result<StoreValue> {
		let StoreValue::Value(value) = value else {
			return Err(anyhow!(
				"Invalid value passed into SQLiteStore GET [{}]",
				value
			));
		};

		self.connection.execute(
			"INSERT INTO KV VALUES(NULL,?1,?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
			[key, &value],
		)?;

		Ok(StoreValue::Value(value))
	}

	fn remove(&mut self, key: &str) -> Result<Option<StoreValue>> {
		let value = self.get(key)?;

		let Some(value) = value else {
			return Ok(None);
		};

		let query = self
			.connection
			.execute("DELETE FROM KV where key = ?1", [key])?;

		if query == 0 {
			panic!("Deleted 0 Rows when trying to delete Value from Store")
		}

		Ok(Some(value))
	}

	fn clear(&mut self) -> Result<usize> {
		let deleted = self.connection.execute("DELETE FROM KV", [])?;
		Ok(deleted)
	}
}
