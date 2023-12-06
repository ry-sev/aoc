use anyhow::Result;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

#[derive(Default, Debug)]
struct Race(usize, usize);

impl Race {
	fn time(&self) -> usize {
		self.0
	}

	fn distance_record(&self) -> usize {
		self.1
	}

	fn distance(&self, hold_time: usize) -> usize {
		self.time()
			.checked_sub(hold_time)
			.map_or(0, |diff| diff.saturating_mul(hold_time))
	}

	fn possible_hold_times(&self) -> Vec<usize> {
		let race_time = self.time();
		let distance_record = self.distance_record();

		(0..=race_time)
			.filter(|&i| self.distance(i) > distance_record)
			.collect()
	}
}

impl From<(usize, usize)> for Race {
	fn from((time, distance): (usize, usize)) -> Self {
		Self(time, distance)
	}
}

fn races_from_file(path: &str) -> Result<Vec<Race>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let mut time = Vec::new();
	let mut distance = Vec::new();

	for line in reader.lines() {
		let line = line?;
		let line = line.trim();

		if line.starts_with("Time:") {
			time = line
				.split_whitespace()
				.skip(1)
				.filter_map(|s| s.parse().ok())
				.collect();
		} else if line.starts_with("Distance:") {
			distance = line
				.split_whitespace()
				.skip(1)
				.filter_map(|s| s.parse().ok())
				.collect();
		}
	}

	let mut races = Vec::new();

	for i in 0..time.len() {
		races.push((time[i], distance[i]).into());
	}

	Ok(races)
}

fn main() -> Result<()> {
	let races = races_from_file("inputs/day_06.txt")?;

	part_one(&races);
	part_two(&races);

	Ok(())
}

fn part_one(races: &[Race]) {
	let margin_of_error: usize = races
		.iter()
		.map(|race| race.possible_hold_times().len())
		.collect::<Vec<_>>()
		.iter()
		.product();

	println!("Part 1: {}", margin_of_error);
}

fn part_two(races: &[Race]) {
	let time: usize = races
		.iter()
		.map(|race| race.time().to_string())
		.collect::<String>()
		.parse()
		.unwrap_or_default();

	let distance: usize = races
		.iter()
		.map(|race| race.distance_record().to_string())
		.collect::<String>()
		.parse()
		.unwrap_or_default();

	let margin_of_error = quadratic_formula(time as f64, distance as f64);

	println!("Part 2: {}", margin_of_error);
}

fn quadratic_formula(hold_duration: f64, distance: f64) -> f64 {
	let discriminant = hold_duration * hold_duration - 4.0 * distance;

	let sqrt_discriminant = discriminant.sqrt();
	let low = (hold_duration - sqrt_discriminant) / 2.0;
	let high = (hold_duration + sqrt_discriminant) / 2.0;

	high.ceil() - low.floor() - 1.
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn time_7() {
		let race = Race(7, 9);
		assert_eq!(race.distance(1), 6);
		assert_eq!(race.distance(2), 10);
		assert_eq!(race.distance(3), 12);
		assert_eq!(race.distance(4), 12);
		assert_eq!(race.distance(5), 10);
		assert_eq!(race.distance(6), 6);
		assert_eq!(race.distance(7), 0);
	}

	#[test]
	fn hold_times() {
		let race = Race(7, 9);
		assert_eq!(race.possible_hold_times(), [2, 3, 4, 5]);
	}
}
