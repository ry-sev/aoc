use anyhow::Result;
use std::{
	collections::HashMap,
	fs::File,
	io::{self, BufRead},
};

fn main() -> Result<()> {
	let file = File::open("inputs/day_04.txt")?;
	let mut card_scores: Vec<u8> = Vec::new();

	for line in io::BufReader::new(file).lines() {
		let line = line?;

		if let Some(pipe_index) = line.find('|') {
			let (winning_numbers, numbers) = line.split_at(pipe_index);

			let winning_numbers: Vec<u8> = winning_numbers
				.split_whitespace()
				.filter_map(|num_str| num_str.parse().ok())
				.collect();

			let numbers: Vec<u8> = numbers[1..]
				.split_whitespace()
				.filter_map(|num_str| num_str.parse().ok())
				.collect();

			let score = numbers
				.iter()
				.filter(|&n| winning_numbers.contains(n))
				.count() as u8;

			card_scores.push(score);
		}
	}

	let store = (0..card_scores.len())
		.map(|index| (index, 1))
		.collect::<HashMap<usize, u32>>();

	let result = card_scores
		.iter()
		.enumerate()
		.fold(store, |mut acc, (index, card_score)| {
			for i in (index + 1)..(index + 1 + *card_score as usize) {
				let to_add = *acc.get(&index).unwrap();
				acc.entry(i).and_modify(|value| *value += to_add);
			}

			acc
		})
		.values()
		.sum::<u32>();

	println!("{:#?}", result);

	Ok(())
}
