use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::fs::{self};

type Graph = BTreeMap<String, (String, String)>;

fn traverse(
	graph: &mut Graph,
	directions: &[char],
	start: &str,
	end: &str,
) -> Result<()> {
	let mut current = start.to_owned();
	let destination = end.to_owned();

	let mut cycle_iter = directions.iter().cycle();

	let mut steps = 0;
	while current != destination {
		let (left, right) = graph
			.get(&current)
			.ok_or_else(|| anyhow!("Invalid node: {current}"))?;

		let direction = cycle_iter
			.next()
			.ok_or_else(|| anyhow!("Direction not found"))?;

		match direction {
			'L' => current = left.to_owned(),
			'R' => current = right.to_owned(),
			_ => return Err(anyhow!("Invalid direction: {direction}")),
		}

		steps += 1;
	}

	println!("{}", steps);

	Ok(())
}

fn main() -> Result<()> {
	let input = fs::read_to_string("inputs/day_08.txt")?;

	let mut graph = BTreeMap::new();

	let (directions, network) = input
		.trim()
		.split_once('\n')
		.expect("There should be a newline");

	let directions: Vec<char> = directions.chars().collect();

	let network: Vec<_> = network.lines().filter(|x| !x.is_empty()).collect();

	for element in network {
		if let Some((node, left_right)) = element.split_once(" =") {
			let trimmed = left_right
				.trim_matches(|c: char| {
					c == '(' || c == ')' || c.is_whitespace()
				})
				.replace(' ', "");

			if let Some((left, right)) = trimmed.split_once(',') {
				graph.insert(
					node.to_owned(),
					(left.to_owned(), right.to_owned()),
				);
			} else {
				return Err(anyhow!("Invalid (LEFT,RIGHT) format"));
			}
		} else {
			return Err(anyhow!("Invalid input format"));
		}
	}

	traverse(&mut graph, &directions, "AAA", "ZZZ")
}
