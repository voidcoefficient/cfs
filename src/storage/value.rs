use std::fmt::Display;

use json::JsonValue;

#[derive(Debug, Clone)]
pub enum StoreValue {
	Value(String),
	List(Vec<String>),
}

impl Display for StoreValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StoreValue::Value(v) => write!(f, "{}", v),
			StoreValue::List(items) => {
				let mut array_string = String::new();
				for i in items.iter() {
					array_string.push_str(&format!("{},", i));
				}

				write!(f, "[{}]", array_string)
			}
		}
	}
}

impl From<StoreValue> for JsonValue {
	fn from(value: StoreValue) -> Self {
		match value {
			StoreValue::Value(string) => JsonValue::String(string),
			StoreValue::List(items) => {
				JsonValue::Array(items.into_iter().map(|i| JsonValue::String(i)).collect())
			}
		}
	}
}

impl From<JsonValue> for StoreValue {
	fn from(value: JsonValue) -> Self {
		match value {
			JsonValue::Array(json_values) => {
				StoreValue::List(json_values.iter().map(|f| f.to_string()).collect())
			}
			JsonValue::String(string) => StoreValue::Value(string),
			_ => StoreValue::Value(value.to_string()),
		}
	}
}

impl From<&JsonValue> for StoreValue {
	fn from(value: &JsonValue) -> Self {
		match value {
			JsonValue::Array(json_values) => {
				StoreValue::List(json_values.iter().map(|f| f.to_string()).collect())
			}
			JsonValue::String(string) => StoreValue::Value(string.clone()),
			_ => StoreValue::Value(value.to_string()),
		}
	}
}
