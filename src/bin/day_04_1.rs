use anyhow::Result;
use std::{
	fs::File,
	io::{self, BufRead},
};

fn main() -> Result<()> {
	let file = File::open("inputs/day_04.txt")?;
	let reader = io::BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines() {
		let line = line?;

		if let Some(pipe_index) = line.find('|') {
			let winning_numbers: Vec<u8> = line[..pipe_index]
				.split_whitespace()
				.filter_map(|num_str| num_str.parse().ok())
				.collect();

			let numbers: Vec<u8> = line[pipe_index + 1..]
				.split_whitespace()
				.filter_map(|num_str| num_str.parse().ok())
				.collect();

			let count_in_winning = numbers
				.iter()
				.filter(|&n| winning_numbers.contains(n))
				.count();

			sum += if count_in_winning == 0 {
				0
			} else {
				2u32.pow(count_in_winning as u32 - 1)
			};
		}
	}

	println!("{}", sum);

	Ok(())
}
