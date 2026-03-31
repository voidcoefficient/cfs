use std::fmt::Display;

use json::JsonValue;

#[derive(Debug, Clone)]
pub enum CfsValue {
	Value(String),
	List(Vec<String>),
}

impl Display for CfsValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CfsValue::Value(v) => write!(f, "{}", v),
			CfsValue::List(items) => {
				let mut array_string = String::new();
				for i in items.iter() {
					array_string.push_str(&format!("{},", i));
				}

				write!(f, "[{}]", array_string)
			}
		}
	}
}

impl From<CfsValue> for JsonValue {
	fn from(value: CfsValue) -> Self {
		match value {
			CfsValue::Value(string) => JsonValue::String(string),
			CfsValue::List(items) => {
				JsonValue::Array(items.into_iter().map(|i| JsonValue::String(i)).collect())
			}
		}
	}
}

impl From<JsonValue> for CfsValue {
	fn from(value: JsonValue) -> Self {
		match value {
			JsonValue::Array(json_values) => {
				CfsValue::List(json_values.iter().map(|f| f.to_string()).collect())
			}
			JsonValue::String(string) => CfsValue::Value(string),
			_ => CfsValue::Value(value.to_string()),
		}
	}
}

impl From<&JsonValue> for CfsValue {
	fn from(value: &JsonValue) -> Self {
		match value {
			JsonValue::Array(json_values) => {
				CfsValue::List(json_values.iter().map(|f| f.to_string()).collect())
			}
			JsonValue::String(string) => CfsValue::Value(string.clone()),
			_ => CfsValue::Value(value.to_string()),
		}
	}
}
