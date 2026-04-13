use seahorse::ActionError;

pub fn invalid(cause: &str) -> ActionError {
	ActionError {
		message: format!(
			"invalid {}. get help by running `{} --help`",
			cause,
			env!("CARGO_PKG_NAME")
		),
	}
}

pub fn to_action_error(err: anyhow::Error) -> ActionError {
	return ActionError {
		message: err.to_string(),
	};
}
