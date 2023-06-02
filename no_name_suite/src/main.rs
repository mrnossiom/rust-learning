const START: &str = "1";

fn main() {
	let mut last = START.to_string();

	for i in 0..100 {
		println!("Chars: {}\nGen: {i}\n{}", last.len(), last);

		last = advance(&last);
	}
}

fn advance(base: &str) -> String {
	let mut chars = base.chars().peekable();
	let mut new = String::new();

	let mut count = 1;
	while let Some(char) = chars.next() {
		if char == *chars.peek().unwrap_or(&' ') {
			count += 1;
		} else {
			new.push_str(&count.to_string());
			new.push(char);
			count = 1;
		}
	}

	new
}
