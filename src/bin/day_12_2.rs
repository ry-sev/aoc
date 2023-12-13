use itertools::{repeat_n, Itertools};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Record {
	springs: Vec<char>,
	unknown: u32,
	groups: Vec<u32>,
}

impl Record {
	fn get_options(&self) -> impl Iterator<Item = String> {
		repeat_n([".", "#"].into_iter(), self.unknown as usize)
			.multi_cartesian_product()
			.map(|v| v.join(""))
	}

	fn check_option(&self, option: &str) -> bool {
		let mut option_iter = option.chars();

		let filled_options: String = self
			.springs
			.iter()
			.map(|&c| match c {
				'?' => option_iter.next().unwrap(),
				_ => c,
			})
			.collect();

		let counts = filled_options
			.chars()
			.group_by(|c| c == &'#')
			.into_iter()
			.filter_map(|(is_hashes, group)| {
				is_hashes.then_some(group.into_iter().count() as u32)
			})
			.collect::<Vec<u32>>();

		self.groups[..] == counts[..]
	}

	fn possible_options(&self) -> usize {
		self.get_options()
			.filter(|option| self.check_option(option))
			.count()
	}
}

fn part_two(input: &str) {
	let sum: usize = input
		.lines()
		.filter_map(|line| {
			let mut parts = line.split_whitespace();
			let springs = parts.next()?.chars().collect::<Vec<_>>();
			let springs = std::iter::repeat(springs)
				.take(5)
				.flatten()
				.collect::<Vec<_>>();

			let groups = parts
				.next()?
				.split(',')
				.filter_map(|s| s.parse().ok())
				.collect::<Vec<u32>>();
			let unknown = springs.iter().filter(|c| **c == '?').count() as u32;

			Some(Record {
				springs,
				groups: std::iter::repeat(groups).take(5).flatten().collect(),
				unknown,
			})
		})
		.map(|record| record.possible_options())
		.sum();

	println!("Sum: {:#?}", sum);
}

fn main() {
	let input = std::fs::read_to_string("inputs/test.txt")
		.expect("Input file should exist");

	part_two(&input);
}
