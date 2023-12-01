use std::{
	fs::File,
	io::{self, BufRead},
};

fn number_to_digit(number: &str) -> Option<u32> {
	match number {
		"zero" => Some(0),
		"one" => Some(1),
		"two" => Some(2),
		"three" => Some(3),
		"four" => Some(4),
		"five" => Some(5),
		"six" => Some(6),
		"seven" => Some(7),
		"eight" => Some(8),
		"nine" => Some(9),
		_ => None,
	}
}

fn main() -> io::Result<()> {
	let mut sum = 0;

	let file = File::open("inputs/day_01.txt")?;
	let reader = io::BufReader::new(file);

	for line in reader.lines() {
		let line = line.unwrap_or_default();

		let mut digits: Vec<u32> = Vec::new();

		for i in 0..line.len() {
			for j in 1..=5 {
				if let Some(substring) = line.get(i..i + j) {
					match j {
						1 => {
							if let Some(character) = substring.chars().next() {
								if let Some(digit) = character.to_digit(10) {
									digits.push(digit);
								}
							}
						}
						_ => {
							if let Some(digit) = number_to_digit(substring) {
								digits.push(digit);
							}
						}
					}
				}
			}
		}

		if digits.len() == 1 {
			digits.push(digits[0]);
		}

		sum += digits[0] * 10 + digits[digits.len() - 1];
	}

	println!("{}", sum);

	Ok(())
}
