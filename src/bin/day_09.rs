fn part_one(input: &str) -> i64 {
	input
		.split_terminator('\n')
		.map(|line| {
			let mut diffs = line
				.split(' ')
				.map(|s| s.parse::<i64>().unwrap())
				.collect::<Vec<_>>();

			let mut numbers = Vec::new();

			while diffs.iter().any(|x| *x != 0) {
				diffs = diffs
					.windows(2)
					.enumerate()
					.map(|(index, window)| {
						match index {
							last if last == diffs.len() - 2 => {
								numbers.push(window[1]);
							}
							_ => {}
						}
						window[1] - window[0]
					})
					.collect();
			}
			numbers.iter().sum::<i64>()
		})
		.sum()
}

fn part_two(input: &str) -> i64 {
	input
		.split_terminator('\n')
		.map(|line| {
			let mut diffs = line
				.split(' ')
				.map(|s| s.parse::<i64>().unwrap())
				.collect::<Vec<_>>();

			let mut numbers = Vec::new();

			while diffs.iter().any(|x| *x != 0) {
				diffs = diffs
					.windows(2)
					.enumerate()
					.map(|(index, window)| {
						if index == 0 {
							numbers.push(window[0]);
						}
						window[1] - window[0]
					})
					.collect();
			}
			numbers.iter().rev().fold(0, |acc, x| x - acc)
		})
		.sum()
}

fn main() {
	let input = std::fs::read_to_string("inputs/day_09.txt")
		.expect("Input file should exist");

	println!("Part 1: {}", part_one(&input));
	println!("Part 2: {}", part_two(&input));
}
