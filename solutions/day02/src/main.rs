use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Day02 {
	lines: Vec<String>,
}

impl<R> From<R> for Day02
where
	R: BufRead,
{
	fn from(reader: R) -> Self {
		let lines = reader
			.lines()
			.map(|line| line.expect("Failed to read line"))
			.collect::<Vec<_>>();

		Day02::new(lines)
	}
}

impl Day02 {
	pub fn new<I, S>(lines: I) -> Day02
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		let lines = lines.into_iter().map(|s| s.into()).collect();

		Day02 { lines }
	}

	fn delta(left: &str, right: &str) -> usize {
		left.chars()
			.zip(right.chars())
			.fold(0, |acc, (a, b)| acc + (a != b) as usize)
	}

	fn diff(left: &str, right: &str) -> String {
		left.chars()
			.zip(right.chars())
			.filter(|(a, b)| a == b)
			.map(|(a, _)| a)
			.collect()
	}

	pub fn checksum(&self) -> usize {
		let (twice, thrice) = self
			.lines
			.iter()
			// Count instances of each character
			.map(|line| {
				line.chars().fold(HashMap::new(), |mut acc, c| {
					*acc.entry(c).or_insert(0) += 1;

					acc
				})
			})
			// Did this map include double or triple occurances?
			.map(|acc| {
				let has_two = acc.values().find(|&&n| n == 2).is_some();
				let has_three = acc.values().find(|&&n| n == 3).is_some();

				(has_two, has_three)
			})
			// Sum the totals
			.fold((0, 0), |(twos, threes), (has_two, has_three)| {
				(twos + has_two as usize, threes + has_three as usize)
			});

		twice * thrice
	}

	pub fn matches<'a>(self: &'a Self) -> impl Iterator<Item = String> + 'a {
		self.lines
			.iter()
			.enumerate()
			.filter_map(move |(i, left)| {
				self.lines[i..]
					.iter()
					.find(|right| Day02::delta(left, right) == 1)
					.map(|right| (left, right))
			})
			.map(|(left, right)| Day02::diff(left, right))
	}
}

fn main() {
	let day: Day02 = io::stdin().lock().into();

	println!("Checksum: {}", day.checksum());
	day.matches().for_each(|m| println!("Match: {}", m));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse() {
		let day: Day02 = "a\nbbb\ncc".as_bytes().into();
		let lines = vec!["a", "bbb", "cc"];

		assert_eq!(day, Day02::new(lines));
	}

	#[test]
	fn checksum() {
		let day = Day02::new(vec![
			"abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee",
			"ababab",
		]);

		assert_eq!(day.checksum(), 12);
	}

	#[test]
	fn matches() {
		let day = Day02::new(vec![
			"abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
		]);

		let matches = day.matches().collect::<Vec<_>>();

		assert_eq!(matches, vec!["fgij"]);
	}
}
