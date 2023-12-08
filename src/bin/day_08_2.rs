use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::fs;

type Graph = BTreeMap<String, (String, String)>;

fn traverse(graph: &Graph, directions: &[char]) -> Result<usize> {
	let starting_nodes: Vec<_> =
		graph.keys().filter(|x| x.ends_with('A')).collect();

	let results: Vec<_> = starting_nodes
		.iter()
		.map(|node| {
			let mut visited = vec![*node];
			let mut current = *node;

			let steps = directions
				.iter()
				.cycle()
				.enumerate()
				.find_map(|(index, direction)| {
					let (left, right) = graph.get(current)?;

					let next = match direction {
						'L' => left,
						'R' => right,
						_ => panic!("Invalid direction"),
					};

					if next.ends_with('Z') {
						Some(index + 1)
					} else {
						visited.push(next);
						current = next;
						None
					}
				})
				.expect("Should find a cycle");

			Some(steps)
		})
		.collect();

	Ok(lcm(&results))
}

fn gcd(a: usize, b: usize) -> usize {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}

fn lcm(numbers: &[Option<usize>]) -> usize {
	numbers
		.iter()
		.cloned()
		.fold(1, |acc, x| acc * x.unwrap_or(1) / gcd(acc, x.unwrap_or(1)))
}

fn main() -> Result<()> {
	let input = fs::read_to_string("inputs/day_08.txt")?;

	let mut graph = BTreeMap::new();

	let (directions, network) = input
		.trim()
		.split_once('\n')
		.ok_or_else(|| anyhow!("Invalid input format"))?;

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

	let min_cycle = traverse(&graph, &directions)?;

	println!("{}", min_cycle);

	Ok(())
}
