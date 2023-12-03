use anyhow::Result;
use regex::Regex;
use std::{
	fs::File,
	io::{self, BufRead},
};

#[derive(Default)]
struct MinCubes {
	r: u32,
	g: u32,
	b: u32,
}

fn main() -> Result<()> {
	let file = File::open("inputs/day_02.txt")?;
	let reader = io::BufReader::new(file);

	let re = Regex::new(r"(?P<amount>\d+)\s(?P<color>green|red|blue)")?;
	let mut sum = 0;

	for (_game, line) in reader.lines().enumerate() {
		let line = line.unwrap_or_default();
		let mut min_cubes = MinCubes::default();

		for (_, [amount, color]) in re.captures_iter(&line).map(|c| c.extract())
		{
			let amount = amount.parse::<u32>()?;

			match color {
				"red" => min_cubes.r = min_cubes.r.max(amount),
				"green" => min_cubes.g = min_cubes.g.max(amount),
				"blue" => min_cubes.b = min_cubes.b.max(amount),
				_ => unreachable!(),
			}
		}
		sum += min_cubes.r * min_cubes.g * min_cubes.b;
	}

	println!("{}", sum);

	Ok(())
}
