use std::path::Path;

use rusqlite::OptionalExtension as _;

use crate::storage::{Store, StoreValue};

#[derive(Debug)]
pub struct SQLiteStore {
	connection: rusqlite::Connection,
}

impl SQLiteStore {
	pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
		let conn = rusqlite::Connection::open(path).expect("To Open SQLite DB");

		conn
			.execute_batch(include_str!("schema.sql"))
			.expect("To Create DB");

		return Self { connection: conn };
	}
}

impl Store for SQLiteStore {
	fn all(&self) -> Vec<(String, super::StoreValue)> {
		let mut stmt = self.connection.prepare("SELECT key,value from KV").unwrap();

		let values = stmt
			.query_map([], |row| {
				let key: String = row.get(0)?;
				let value = StoreValue::Value(row.get::<_, String>(1)?);

				return Ok((key, value));
			})
			.unwrap()
			.collect::<Result<Vec<_>, _>>()
			.unwrap();

		return values;
	}

	fn get(&self, key: &str) -> Option<super::StoreValue> {
		let stmt = self
			.connection
			.query_row(
				"SELECT key,value from KV where key = ?1 LIMIT 1",
				[key],
				|row| Ok(StoreValue::Value(row.get(1).unwrap())),
			)
			.optional()
			.unwrap();

		return stmt;
	}

	fn set(&mut self, key: &str, value: super::StoreValue) -> super::StoreValue {
		let StoreValue::Value(value) = value else {
			panic!("Invalid value passed into SQLiteStore GET [{}]", value)
		};

		self
			.connection
			.execute(
				"INSERT INTO KV VALUES(NULL,?1,?2) ON CONFLICT(key) DO UPDATE SET value = ?2 WHERE key = ?1",
				[key, &value],
			)
			.unwrap();

		return StoreValue::Value(value);
	}

	fn remove(&mut self, key: &str) -> Option<super::StoreValue> {
		let value = self.get(key);

		let Some(value) = value else {
			return None;
		};

		let stmt = self
			.connection
			.execute("DELETE FROM KV where key = ?1", [key])
			.unwrap();

		if stmt == 0 {
			panic!("Deleted 0 Rows when trying to delete Value from Store")
		}

		Some(value)
	}

	fn clear(&mut self) {
		self.connection.execute("DELETE FROM KV", []).unwrap();
	}
}
