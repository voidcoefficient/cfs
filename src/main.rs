use std::{env, io};

use seahorse::App;

use crate::commands::{clear, get_value, init, list, remove_value, set_value};

mod actions;
mod commands;
mod config;
mod error;
mod flags;
mod json_object;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let app = App::new(env!("CARGO_PKG_NAME"))
		.description(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage(format!("{} [commands]", env!("CARGO_PKG_NAME")))
		.command(set_value())
		.command(get_value())
		.command(list())
		.command(init())
		.command(remove_value())
		.command(clear());

	app.run(args);

	Ok(())
}
