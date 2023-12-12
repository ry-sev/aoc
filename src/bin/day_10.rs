use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy)]
enum Direction {
	North,
	South,
	East,
	West,
}

fn part_one(bytes: &[u8]) -> Result<usize> {
	let width = bytes
		.iter()
		.position(|&b| b == b'\n')
		.ok_or(anyhow!("Invalid map: newline not found"))?;

	let start = bytes
		.iter()
		.position(|&b| b == b'S')
		.ok_or(anyhow!("Invalid map: starting position not found"))?;

	let (mut position, mut direction) = {
		if matches!(bytes[start - width - 1], b'|' | b'7' | b'F') {
			(start - width - 1, Direction::North)
		} else if matches!(bytes[start + width + 1], b'|' | b'L' | b'J') {
			(start + width + 1, Direction::South)
		} else {
			(start - 1, Direction::West)
		}
	};

	Ok((1 + std::iter::repeat(())
		.position(|_| {
			if let Some(pipe) = bytes.get(position) {
				match (pipe, direction) {
					(b'|', Direction::South) => position += width + 1,
					(b'|', Direction::North) => position -= width + 1,
					(b'-', Direction::West) => position -= 1,
					(b'-', Direction::East) => position += 1,
					(b'L', Direction::South) | (b'F', Direction::North) => {
						position += 1;
						direction = Direction::East;
					}
					(b'L', Direction::West) | (b'J', Direction::East) => {
						position -= width + 1;
						direction = Direction::North;
					}
					(b'7', Direction::North) | (b'J', Direction::South) => {
						position -= 1;
						direction = Direction::West;
					}
					(b'7', Direction::East) | (b'F', Direction::West) => {
						position += width + 1;
						direction = Direction::South;
					}
					(b'S', _) => return true,
					(_, _) => unreachable!(),
				}
				false
			} else {
				true
			}
		})
		.ok_or(anyhow!("Invalid map: end condition not reached"))?)
		/ 2)
}

fn part_two(bytes: &[u8]) -> Result<usize> {
	let width = bytes
		.iter()
		.position(|&b| b == b'\n')
		.ok_or(anyhow!("Invalid map: newline not found"))?;

	let start = bytes
		.iter()
		.position(|&b| b == b'S')
		.ok_or(anyhow!("Invalid map: starting position not found"))?;

	let mut pipes = vec![false; bytes.len()];

	let (mut position, mut direction) = {
		if matches!(bytes[start - width - 1], b'|' | b'7' | b'F') {
			(start - width - 1, Direction::North)
		} else if matches!(bytes[start + width + 1], b'|' | b'L' | b'J') {
			(start + width + 1, Direction::South)
		} else {
			(start - 1, Direction::West)
		}
	};

	std::iter::repeat(())
		.position(|_| {
			if let Some(pipe) = pipes.get_mut(position) {
				*pipe = true;
			} else {
				panic!("Index out of bounds");
			}
			if let Some(pipe) = bytes.get(position) {
				match (pipe, direction) {
					(b'|', Direction::South) => position += width + 1,
					(b'|', Direction::North) => position -= width + 1,
					(b'-', Direction::West) => position -= 1,
					(b'-', Direction::East) => position += 1,
					(b'L', Direction::South) | (b'F', Direction::North) => {
						position += 1;
						direction = Direction::East;
					}
					(b'L', Direction::West) | (b'J', Direction::East) => {
						position -= width + 1;
						direction = Direction::North;
					}
					(b'7', Direction::North) | (b'J', Direction::South) => {
						position -= 1;
						direction = Direction::West;
					}
					(b'7', Direction::East) | (b'F', Direction::West) => {
						position += width + 1;
						direction = Direction::South;
					}
					(b'S', _) => return true,
					(_, _) => unreachable!(),
				}
				false
			} else {
				true
			}
		})
		.ok_or(anyhow!("Invalid map: end condition not reached"))?;

	let mut inside = false;

	Ok(bytes
		.iter()
		.enumerate()
		.filter(|(position, tile)| {
			let is_pipe = *pipes.get(*position).unwrap_or(&false);
			inside &= position % (width + 1) != 0;
			inside ^= is_pipe && matches!(*tile, b'|' | b'F' | b'7');
			inside
				&& (!is_pipe || **tile == b'.')
				&& (position % (width + 1) != width)
		})
		.count())
}

fn main() -> Result<()> {
	let bytes = include_bytes!("../../inputs/day_10.txt");
	println!("Part 1: {}", part_one(bytes)?);
	println!("Part 2: {}", part_two(bytes)?);
	Ok(())
}
