use itertools::{repeat_n, Itertools};

#[derive(Debug)]
struct Record<'a> {
	springs: &'a str,
	unknown: u8,
	groups: Vec<usize>,
}

impl Record<'_> {
	fn get_options(&self) -> impl Iterator<Item = String> {
		repeat_n([".", "#"].into_iter(), self.unknown as usize)
			.multi_cartesian_product()
			.map(|v| v.join(""))
	}

	fn check_option(&self, option: &str) -> bool {
		let mut option_iter = option.chars();

		let filled_options = self
			.springs
			.chars()
			.map(|c| match c {
				'?' => option_iter.next().unwrap(),
				_ => c,
			})
			.collect::<String>();

		let counts = filled_options
			.chars()
			.group_by(|c| c == &'#')
			.into_iter()
			.filter_map(|(is_hashes, group)| {
				is_hashes.then_some(group.into_iter().count())
			})
			.collect::<Vec<usize>>();

		self.groups[..] == counts[..]
	}

	fn possible_options(&self) -> usize {
		self.get_options()
			.filter(|option| self.check_option(option))
			.count()
	}
}

fn part_one(input: &str) {
	let sum: usize = input
		.lines()
		.filter_map(|line| {
			let mut parts = line.split_whitespace();
			let springs = parts.next()?;
			let groups = parts
				.next()?
				.split(',')
				.filter_map(|s| s.parse().ok())
				.collect::<Vec<usize>>();
			let unknown =
				springs.chars().counts().get(&'?').copied().unwrap_or(0) as u8;

			Some(Record {
				springs,
				groups,
				unknown,
			})
		})
		.map(|record| record.possible_options())
		.sum();

	println!("Sum: {:#?}", sum);
}

fn main() {
	let input = std::fs::read_to_string("inputs/day_12.txt")
		.expect("Input file should exist");

	part_one(&input);
}
