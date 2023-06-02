use std::{
	env,
	io::{stdin, stdout, Write},
	path::Path,
	process::{Child, Command, Stdio},
};

fn main() {
	loop {
		print!("❯ ");
		stdout().flush().unwrap();

		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();

		let mut commands = input.trim().split(" | ").peekable();
		let mut previous_command = None;

		while let Some(command) = commands.next() {
			let mut parts = command.split_whitespace();
			let command = parts.next().unwrap();
			let args = parts;

			match command {
				"cd" => {
					let path = args.peekable().peek().map_or("/", |x| *x);
					let absolute_path = Path::new(path);

					if let Err(error) = env::set_current_dir(absolute_path) {
						eprintln!("{}", error);
					}

					previous_command = None;
				}
				"exit" => return,
				command => {
					let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
						Stdio::from(output.stdout.unwrap())
					});

					let stdout = if commands.peek().is_some() {
						Stdio::piped()
					} else {
						Stdio::inherit()
					};

					let child = Command::new(command)
						.args(args)
						.stdin(stdin)
						.stdout(stdout)
						.spawn();

					match child {
						Ok(child) => {
							previous_command = Some(child);
						}
						Err(error) => {
							previous_command = None;
							eprintln!("{}", error);
						}
					};
				}
			}
		}

		if let Some(mut final_command) = previous_command {
			final_command.wait().unwrap();
		}
	}
}
