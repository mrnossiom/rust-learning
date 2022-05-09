use std::cmp::Ordering;
use std::io::stdin;
use std::process::exit;

fn main() {
	println!("Quel nombre de la suite de Fibonacci voulez vous obtenir ?");

	const MAX_FIBONACCI_NUMBER: u128 = 185;
	let mut user_number = String::new();

	stdin()
		.read_line(&mut user_number)
		.expect("Un problème est survenu...");

	let user_number: u128 = match user_number.trim().parse() {
		Ok(number) => number,
		Err(_) => {
			eprintln!("Vous devez entrez un nombre !");
			exit(1);
		}
	};

	if let Ordering::Greater = user_number.cmp(&MAX_FIBONACCI_NUMBER) {
		println!("Ce nombre est trop grand !");
		exit(1);
	}

	let mut current_number: u128 = 0;
	let mut next_number: u128 = 1;

	for _ in 0..(user_number) {
		println!("{}", &current_number);

		let tmp = next_number;
		next_number += current_number;
		current_number = tmp;
	}

	println!("Le {}ème nombre est {}", &user_number, &current_number);
}
