use anyhow::Result;
use std::{
	fs::File,
	io::{self, BufRead},
};

fn number_to_left(chars: &[char], index: usize) -> Result<u32> {
	let mut number_chars = Vec::new();

	for i in (0..index).rev() {
		let c = chars[i];

		if c.is_ascii_digit() {
			number_chars.push(c);
		} else {
			break;
		}
	}

	if !number_chars.is_empty() {
		number_chars.reverse();
		let number = number_chars.iter().collect::<String>();
		Ok(number.parse::<u32>()?)
	} else {
		Ok(0)
	}
}

fn number_to_right(chars: &[char], index: usize) -> Result<u32> {
	let mut number_chars = Vec::new();

	for c in chars.iter().skip(index + 1) {
		if c.is_ascii_digit() {
			number_chars.push(*c);
		} else {
			break;
		}
	}

	if !number_chars.is_empty() {
		let number = number_chars.iter().collect::<String>();
		Ok(number.parse::<u32>()?)
	} else {
		Ok(0)
	}
}

fn number_span(chars: &[char], index: usize) -> Result<u32> {
	let mut left_index = index;

	while left_index > 0 && chars[left_index - 1].is_ascii_digit() {
		left_index -= 1;
	}

	let mut right_index = index;
	while right_index < chars.len() - 1
		&& chars[right_index + 1].is_ascii_digit()
	{
		right_index += 1;
	}

	let number = chars[left_index..=right_index].iter().collect::<String>();

	Ok(number.parse::<u32>()?)
}

fn main() -> Result<()> {
	let file = File::open("inputs/day_03.txt")?;
	let reader = io::BufReader::new(file);

	let matrix: Vec<Vec<char>> = reader
		.lines()
		.map(|line| line.expect("Line to exist").chars().collect())
		.collect();

	let mut sum = 0;

	for (row_index, row) in matrix.iter().enumerate() {
		for (item_index, item) in row.iter().enumerate() {
			if item == &'.' || item.is_ascii_digit() {
				continue;
			}

			sum += number_to_left(row, item_index)?;
			sum += number_to_right(row, item_index)?;

			if let Some(top) = matrix.get(row_index - 1) {
				if !top[item_index].is_ascii_digit() {
					sum += number_to_left(top, item_index)?;
					sum += number_to_right(top, item_index)?;
				} else {
					sum += number_span(top, item_index)?;
				}
			}

			if let Some(bottom) = matrix.get(row_index + 1) {
				if !bottom[item_index].is_ascii_digit() {
					sum += number_to_left(bottom, item_index)?;
					sum += number_to_right(bottom, item_index)?;
				} else {
					sum += number_span(bottom, item_index)?;
				}
			}
		}
	}

	println!("{}", sum);

	Ok(())
}
