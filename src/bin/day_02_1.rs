use anyhow::Result;
use regex::Regex;
use std::{
	fs::File,
	io::{self, BufRead},
};

const MAX_VALUES: [u32; 3] = [12, 13, 14];

fn color_index(color: &str) -> usize {
	match color {
		"red" => 0,
		"green" => 1,
		"blue" => 2,
		_ => unreachable!(),
	}
}

fn main() -> Result<()> {
	let file = File::open("inputs/day_02.txt")?;
	let reader = io::BufReader::new(file);

	let re = Regex::new(r"(?P<amount>\d+)\s(?P<color>green|red|blue)")?;

	let sum: u32 = reader
		.lines()
		.enumerate()
		.filter_map(|(game, line)| {
			let line = line.unwrap_or_default();

			let valid_game = re.captures_iter(&line).all(|c| {
				let amount = c["amount"]
					.parse::<u32>()
					.expect("Amount should be a number");
				let color = &c["color"];

				match color {
					"red" | "green" | "blue" => {
						amount <= MAX_VALUES[color_index(color)]
					}
					_ => false,
				}
			});

			if valid_game {
				Some((game + 1) as u32)
			} else {
				None
			}
		})
		.collect::<Vec<u32>>()
		.iter()
		.sum();

	println!("{}", sum);

	Ok(())
}
