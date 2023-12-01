use std::{
	fs::File,
	io::{self, BufRead},
};

fn main() -> io::Result<()> {
	let file = File::open("inputs/day_01.txt")?;
	let reader = io::BufReader::new(file);

	let sum: u32 = reader
		.lines()
		.map(|line| {
			line.unwrap_or_default()
				.chars()
				.filter_map(|c| c.to_digit(10))
				.collect::<Vec<_>>()
		})
		.filter(|digits| !digits.is_empty())
		.map(|digits| digits[0] * 10 + digits[digits.len() - 1])
		.sum();

	println!("{}", sum);

	Ok(())
}
