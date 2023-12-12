fn part_one(bytes: &[u8]) -> usize {
	let size = bytes.iter().position(|&c| c == b'\n').unwrap();
	let (x, y) = count_occurrences(bytes, size);
	dist(&x, 1) + dist(&y, 1)
}

fn part_two(bytes: &[u8]) -> usize {
	let size = bytes.iter().position(|&c| c == b'\n').unwrap();
	let (x, y) = count_occurrences(bytes, size);
	dist(&x, 999_999) + dist(&y, 999_999)
}

fn count_occurrences(bytes: &[u8], size: usize) -> (Vec<usize>, Vec<usize>) {
	let mut x = vec![0; size];
	let mut y = vec![0; size];

	for (pos, _) in bytes.iter().enumerate().filter(|&(_, &b)| b == b'#') {
		x[pos % (size + 1)] += 1;
		y[pos / (size + 1)] += 1;
	}

	(x, y)
}

fn dist(counts: &[usize], inc: usize) -> usize {
	counts
		.iter()
		.enumerate()
		.fold((0, 0, 0, 0), |(gaps, sum, items, dist), (i, &count)| {
			if count > 0 {
				let expanded = i + inc * gaps;
				(
					gaps,
					sum + count * expanded,
					items + count,
					dist + count * (items * expanded - sum),
				)
			} else {
				(gaps + 1, sum, items, dist)
			}
		})
		.3
}

pub fn main() {
	let bytes = include_bytes!("../../inputs/day_11.txt");
	println!("Part 1: {}", part_one(bytes));
	println!("Part 2: {}", part_two(bytes));
}
