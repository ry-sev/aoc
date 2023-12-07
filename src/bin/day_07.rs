use anyhow::Result;
use itertools::{Itertools, Position};
use std::{
	fs::{self},
	ops::Deref,
};

#[derive(Debug, Clone, Copy)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

fn score_hand(
	hand: &str,
	with_joker: bool,
) -> (HandType, (u32, u32, u32, u32, u32)) {
	let counts = hand.chars().counts();
	let values = if with_joker {
		counts.get(&'J').map_or_else(
			|| counts.values().sorted().join(""),
			|joker_count| {
				if *joker_count == 5 {
					"5".to_string()
				} else {
					counts
						.iter()
						.filter_map(|(key, value)| {
							(key != &'J').then_some(value)
						})
						.sorted()
						.with_position()
						.map(|(position, value)| match position {
							Position::Last | Position::Only => {
								value + joker_count
							}
							_ => *value,
						})
						.join("")
				}
			},
		)
	} else {
		counts.values().sorted().join("")
	};

	let hand_type = match values.deref() {
		"5" => HandType::FiveOfAKind,
		"14" => HandType::FourOfAKind,
		"23" => HandType::FullHouse,
		"113" => HandType::ThreeOfAKind,
		"122" => HandType::TwoPair,
		"1112" => HandType::OnePair,
		"11111" => HandType::HighCard,
		value => {
			panic!("Could not parse hand to hand type: `{}`", value)
		}
	};

	let card_scores = hand
		.chars()
		.map(|card| match card {
			'A' => 14,
			'K' => 13,
			'Q' => 12,
			'J' => {
				if with_joker {
					1
				} else {
					11
				}
			}
			'T' => 10,
			value => value.to_digit(10).unwrap(),
		})
		.collect_tuple()
		.unwrap();

	(hand_type, card_scores)
}

fn process(input: &str, with_joker: bool) -> u32 {
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(|line| {
			let (hand, bid) =
				line.split_once(' ').expect("Invalid input format");
			(
				hand,
				bid.parse::<u32>().unwrap(),
				score_hand(hand, with_joker),
			)
		})
		.sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
		.enumerate()
		.map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
		.sum::<u32>()
}

fn main() -> Result<()> {
	let input = fs::read_to_string("inputs/day_07.txt")?;

	println!("Part 1: {}", process(&input, false));
	println!("Part 2: {}", process(&input, true));

	Ok(())
}
