use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Day12 {
	initial: String,
	directions: HashMap<String, String>,
}

impl FromStr for Day12 {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut lines = s
			.lines()
			.map(|line| line.trim())
			.filter(|line| line.len() > 0);

		let initial = lines
			.next()
			.ok_or_else(|| "Expected at least one line")?
			.split_whitespace()
			.skip(2)
			.next()
			.ok_or_else(|| "Expected initial state")?
			.to_owned();

		let directions: HashMap<String, String> = lines
			.map(|line| {
				let mut parts = line.split_whitespace();
				let from = parts.next().ok_or_else(|| "Expected from map")?;
				let to =
					parts.skip(1).next().ok_or_else(|| "Expected to map")?;

				Ok((from.to_owned(), to.to_owned()))
			})
			.collect::<Result<_>>()?;

		if directions.get(".....") == Some(&"#".to_owned()) {
			return Err("Unable to create plans from thin air!".into());
		}

		Ok(Self {
			initial,
			directions,
		})
	}
}

impl Day12 {
	fn generation(&self, s: String) -> String {
		s.chars()
			.enumerate()
			.map(|(i, _)| {
				if i < 2 || i + 3 > s.len() {
					return ".".to_owned();
				}

				let sub = &s[(i - 2)..(i + 3)];
				let d = self
					.directions
					.get(sub)
					.map(|s| s.to_owned())
					.unwrap_or_else(|| ".".to_owned());

				d.to_owned()
			})
			.collect()
	}

	fn iteration(&self, n: usize) -> String {
		let pad = ".".repeat(n + 5);
		let initial = pad.clone() + &self.initial + &pad;

		(0..n).fold(initial, |acc, _| self.generation(acc))
	}

	fn count(&self, n: usize) -> i64 {
		self.iteration(n)
			.chars()
			.enumerate()
			.map(|(i, c)| match c {
				'#' => i as i64 - n as i64 - 5,
				_ => 0,
			})
			.sum()
	}
}

fn main() {
	let input = {
		let mut buf = String::new();

		io::stdin()
			.read_to_string(&mut buf)
			.expect("Failed to read stdin");

		buf
	};

	let day: Day12 = input.parse().expect("Failed to parse input");

	println!("Number of plants after 20 iterations: {}", day.count(20));

	println!("Number of plants after 500 iterations: {}", day.count(500));
	println!(
		"Number of plants after 5000 iterations: {}",
		day.count(5000)
	);
	println!(
		"Number of plants after 50000 iterations: {}",
		day.count(50000)
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	static TEST_INPUT: &'static str = r#"
        initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #
    "#;

	#[test]
	fn count() {
		let day: Day12 = TEST_INPUT.parse().unwrap();

		assert_eq!(day.count(20), 325);
	}
}
