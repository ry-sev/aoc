use anyhow::Result;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	ops::{Index, IndexMut, Range},
};

type Seed = u64;

const MAP_TYPES: [&str; 7] = [
	"seed-to-soil map:",
	"soil-to-fertilizer map:",
	"fertilizer-to-water map:",
	"water-to-light map:",
	"light-to-temperature map:",
	"temperature-to-humidity map:",
	"humidity-to-location map:",
];

#[derive(Default, Debug)]
struct Entry {
	destination_range: Range<Seed>,
	source_range: Range<Seed>,
}

impl Entry {
	fn new(destination_range: Range<Seed>, source_range: Range<Seed>) -> Self {
		Self {
			destination_range,
			source_range,
		}
	}

	fn contains_source(&self, source: Seed) -> bool {
		self.source_range.contains(&source)
	}
}

#[derive(Default, Debug)]
struct Map {
	entries: Vec<Entry>,
}

impl Map {
	fn new(entries: Vec<Entry>) -> Self {
		Self { entries }
	}

	fn get(&self, source: Seed) -> Seed {
		self.entries
			.iter()
			.find_map(|entry| {
				if entry.contains_source(source) {
					Some(
						entry.destination_range.start
							+ (source - entry.source_range.start),
					)
				} else {
					None
				}
			})
			.unwrap_or(source)
	}
}

#[derive(Default, Debug)]
struct Almanac {
	seeds: Vec<Seed>,
	seed_to_soil: Map,
	soil_to_fertilizer: Map,
	ferilizer_to_water: Map,
	water_to_light: Map,
	light_to_temperature: Map,
	temperature_to_humidity: Map,
	humidity_to_location: Map,
}

impl Index<usize> for Almanac {
	type Output = Map;

	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.seed_to_soil,
			1 => &self.soil_to_fertilizer,
			2 => &self.ferilizer_to_water,
			3 => &self.water_to_light,
			4 => &self.light_to_temperature,
			5 => &self.temperature_to_humidity,
			6 => &self.humidity_to_location,
			_ => panic!("Index out of bounds for Almanac"),
		}
	}
}

impl IndexMut<usize> for Almanac {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.seed_to_soil,
			1 => &mut self.soil_to_fertilizer,
			2 => &mut self.ferilizer_to_water,
			3 => &mut self.water_to_light,
			4 => &mut self.light_to_temperature,
			5 => &mut self.temperature_to_humidity,
			6 => &mut self.humidity_to_location,
			_ => panic!("Index out of bounds for Almanac"),
		}
	}
}

impl Almanac {
	fn from_file(path: &str) -> Self {
		let file = File::open(path).unwrap();
		let reader = BufReader::new(file);

		let mut almanac = Almanac::default();

		let mut iter = reader.lines().peekable();

		while let Some(Ok(line)) = iter.next() {
			let line = line.trim();
			if line.starts_with("seeds:") {
				almanac.seeds = line
					.split_whitespace()
					.skip(1)
					.filter_map(|s| s.parse().ok())
					.collect();
			} else {
				for (index, map_type) in MAP_TYPES.into_iter().enumerate() {
					if line.starts_with(map_type) {
						let mut parameters = Vec::new();

						while let Some(Ok(line)) = iter.next() {
							let line = line.trim();

							if line.is_empty() || MAP_TYPES.contains(&line) {
								break;
							}

							let seeds: Vec<Seed> = line
								.split_whitespace()
								.filter_map(|s| s.parse().ok())
								.collect();

							if seeds.len() == 3 {
								let destination_range = seeds[0];
								let source_range = seeds[1];
								let range_length = seeds[2];

								let parameter = Entry::new(
									destination_range
										..(destination_range + range_length),
									source_range..(source_range + range_length),
								);
								parameters.push(parameter);
							}
						}

						let map = Map::new(parameters);
						almanac[index] = map;
					}
				}
			}
		}

		almanac
	}

	fn location_from_seed(&self, seed: Seed) -> Seed {
		let soil = self.seed_to_soil.get(seed);
		let fertilizer = self.soil_to_fertilizer.get(soil);
		let water = self.ferilizer_to_water.get(fertilizer);
		let light = self.water_to_light.get(water);
		let temperature = self.light_to_temperature.get(light);
		let humidity = self.temperature_to_humidity.get(temperature);
		self.humidity_to_location.get(humidity)
	}

	fn lowest_location_from_seeds(&self) -> Seed {
		let mut lowest: Seed = Seed::MAX;

		for seed in self.seeds.iter() {
			let location = self.location_from_seed(*seed);
			lowest = lowest.min(location);
		}

		lowest
	}

	fn lowest_location_from_seeds_range(&self) -> Seed {
		let mut lowest: Seed = Seed::MAX;

		let chunks: Vec<_> = self.seeds.chunks(2).collect();

		for chunk in chunks {
			let seed_range = chunk[0]..(chunk[0] + chunk[1]);
			for seed in seed_range {
				let location = self.location_from_seed(seed);
				lowest = lowest.min(location);
			}
		}
		lowest
	}
}

fn main() -> Result<()> {
	let almanac = Almanac::from_file("inputs/day_05.txt");

	let part_one = almanac.lowest_location_from_seeds();
	let part_two = almanac.lowest_location_from_seeds_range();

	println!("Part 1: {}", part_one);
	println!("Part 2: {}", part_two);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn seed_79() {
		let almanac = Almanac::from_file("inputs/test.txt");
		assert_eq!(almanac.seed_to_soil.get(79), 81);
		assert_eq!(almanac.soil_to_fertilizer.get(81), 81);
		assert_eq!(almanac.ferilizer_to_water.get(81), 81);
		assert_eq!(almanac.water_to_light.get(81), 74);
		assert_eq!(almanac.light_to_temperature.get(74), 78);
		assert_eq!(almanac.temperature_to_humidity.get(78), 78);
		assert_eq!(almanac.humidity_to_location.get(78), 82);
	}

	#[test]
	fn seed_14() {
		let almanac = Almanac::from_file("inputs/test.txt");
		assert_eq!(almanac.seed_to_soil.get(14), 14);
		assert_eq!(almanac.soil_to_fertilizer.get(14), 53);
		assert_eq!(almanac.ferilizer_to_water.get(53), 49);
		assert_eq!(almanac.water_to_light.get(49), 42);
		assert_eq!(almanac.light_to_temperature.get(42), 42);
		assert_eq!(almanac.temperature_to_humidity.get(42), 43);
		assert_eq!(almanac.humidity_to_location.get(43), 43);
	}

	#[test]
	fn seed_55() {
		let almanac = Almanac::from_file("inputs/test.txt");
		assert_eq!(almanac.seed_to_soil.get(55), 57);
		assert_eq!(almanac.soil_to_fertilizer.get(57), 57);
		assert_eq!(almanac.ferilizer_to_water.get(57), 53);
		assert_eq!(almanac.water_to_light.get(53), 46);
		assert_eq!(almanac.light_to_temperature.get(46), 82);
		assert_eq!(almanac.temperature_to_humidity.get(82), 82);
		assert_eq!(almanac.humidity_to_location.get(82), 86);
	}

	#[test]
	fn seed_13() {
		let almanac = Almanac::from_file("inputs/test.txt");
		assert_eq!(almanac.seed_to_soil.get(13), 13);
		assert_eq!(almanac.soil_to_fertilizer.get(13), 52);
		assert_eq!(almanac.ferilizer_to_water.get(52), 41);
		assert_eq!(almanac.water_to_light.get(41), 34);
		assert_eq!(almanac.light_to_temperature.get(34), 34);
		assert_eq!(almanac.temperature_to_humidity.get(34), 35);
		assert_eq!(almanac.humidity_to_location.get(35), 35);
	}
}
