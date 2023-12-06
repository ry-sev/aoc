use anyhow::{Error, Result};
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn parse_file(path: &str) -> Result<(Vec<usize>, Vec<usize>)> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let (time, distance) = reader.lines().try_fold(
		(Vec::new(), Vec::new()),
		|(mut times, mut distances), line| {
			let line = line?;
			let line = line.trim();

			if line.starts_with("Time:") {
				times = line
					.split_whitespace()
					.skip(1)
					.filter_map(|s| s.parse().ok())
					.collect();
			} else if line.starts_with("Distance:") {
				distances = line
					.split_whitespace()
					.skip(1)
					.filter_map(|s| s.parse().ok())
					.collect();
			}

			Ok::<_, Error>((times, distances))
		},
	)?;

	Ok((time, distance))
}

fn main() -> Result<()> {
	let (times, distances) = parse_file("inputs/day_06.txt")?;

	part_one(&times, &distances);
	part_two(&times, &distances)?;

	Ok(())
}

fn part_one(times: &[usize], distances: &[usize]) {
	let margin_of_error =
		times
			.iter()
			.zip(distances)
			.fold(1, |acc, (&time, &distance)| {
				acc * quadratic_formula(time as f64, distance as f64) as usize
			});

	println!("Part 1: {}", margin_of_error);
}

fn part_two(times: &[usize], distances: &[usize]) -> Result<()> {
	let time: usize = times.iter().copied().try_fold(0, |acc, x| {
		Ok::<_, Error>(acc * 10usize.pow((x as f64).log10().ceil() as u32) + x)
	})?;

	let distance: usize = distances.iter().copied().try_fold(0, |acc, x| {
		Ok::<_, Error>(acc * 10usize.pow((x as f64).log10().ceil() as u32) + x)
	})?;

	let margin_of_error = quadratic_formula(time as f64, distance as f64);

	println!("Part 2: {}", margin_of_error);

	Ok(())
}

fn quadratic_formula(hold_duration: f64, distance: f64) -> f64 {
	let discriminant = (hold_duration * hold_duration - 4.0 * distance).sqrt();
	let low = (hold_duration - discriminant) / 2.0;
	let high = (hold_duration + discriminant) / 2.0;
	high.ceil() - low.floor() - 1.
}
