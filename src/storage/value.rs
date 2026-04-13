use std::fmt::Display;

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
