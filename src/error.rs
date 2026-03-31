use std::process::exit;

pub fn invalid(cause: &str) {
	eprintln!(
		"invalid {}. get help by running `{} --help`",
		cause,
		env!("CARGO_PKG_NAME")
	);
	exit(1);
}
