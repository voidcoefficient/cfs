use std::process::exit;

pub fn invalid(cause: &str) {
    eprintln!("invalid {}. get help by running `conf set --help`", cause);
    exit(1);
}
