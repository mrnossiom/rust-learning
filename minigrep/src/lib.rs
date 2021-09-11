use atty::Stream;
use std::{
	env,
	error::Error,
	fs,
	io::{stdin, ErrorKind, Read},
};

pub struct Config {
	query: String,
	filename: Option<String>,
	case_sensitive: bool,
}

impl Config {
	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		args.next();

		let query = match args.next() {
			Some(argument) => argument,
			None => return Err("didn't get a query string"),
		};

		let filename: Option<String> = match args.next() {
			Some(argument) => Some(argument),
			None => {
				if atty::is(Stream::Stdin) {
					return Err(
						"you must supply content either by piping into the program or giving a file name",
					);
				};
				None
			}
		};

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config {
			query,
			filename,
			case_sensitive,
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = get_content(&config)?;

	let results = if config.case_sensitive {
		search(&config.query, &content)
	} else {
		search_case_insensitive(&config.query, &content)
	};

	for line in results {
		println!("{}", line)
	}

	Ok(())
}

fn get_content(config: &Config) -> Result<String, &str> {
	let mut content = String::new();

	if let Some(filename) = &config.filename {
		content = match fs::read_to_string(&filename) {
			Ok(file) => file,
			Err(err) => match err.kind() {
				ErrorKind::NotFound => {
					panic!("File {:?} does not exist.", String::from(filename))
				}
				other_error => panic!("An error occurred while reading file: {:?}", other_error),
			},
		};
	} else {
		stdin()
			.read_to_string(&mut content)
			.expect("Error while reading from stdin");
	};

	Ok(content)
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	content
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();

	content
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
