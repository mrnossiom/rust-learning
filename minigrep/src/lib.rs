use std::{env, error::Error, fs, io::ErrorKind};

pub struct Config {
	query: String,
	filename: String,
	case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config {
			query,
			filename,
			case_sensitive,
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = match fs::read_to_string(&config.filename) {
		Ok(file) => file,
		Err(err) => match err.kind() {
			ErrorKind::NotFound => {
				panic!("File {:?} does not exist.", String::from(&config.filename))
			}
			other_error => panic!("An error occurred while reading file: {:?}", other_error),
		},
	};

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

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in content.lines() {
		if line.contains(query) {
			results.push(line)
		}
	}

	results
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line)
		}
	}

	results
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
