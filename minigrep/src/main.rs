use minigrep::{run, Config};
use std::{env, process};

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.remove(0);

	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = run(config) {
		eprintln!("minigrep error: {}", e);

		process::exit(1)
	}
}
