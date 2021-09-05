use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::process::exit;

fn main() {
	let mut game_succeed = false;
	let number_to_guess = rand::thread_rng().gen_range(1, 101);

	println!("Le nombre secret est {}", &number_to_guess);
	println!("Entrez un nombre: ");

	for _ in 1..11 {
		let mut supposition = String::new();

		stdin()
			.read_line(&mut supposition)
			.expect("Un problème est survenu");

		let supposition: u32 = match supposition.trim().parse() {
			Ok(number) => number,
			Err(_) => {
				println!("Vous devez entrez un nombre !");
				continue;
			}
		};

		match supposition.cmp(&number_to_guess) {
			Ordering::Less => println!("C'est plus..."),
			Ordering::Greater => println!("C'est moins..."),
			Ordering::Equal => {
				println!("Vous avez gagné !");
				game_succeed = true;
				break;
			}
		}
	}

	match game_succeed {
		true => exit(0),
		false => exit(1),
	}
}
