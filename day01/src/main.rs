use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Day01 {
	nums: Vec<i64>,
}

impl<R> From<R> for Day01
where
	R: BufRead,
{
	fn from(reader: R) -> Self {
		let nums = reader
			.lines()
			.map(|line| line.expect("Failed to read line"))
			.map(|line| line.parse::<i64>().expect("Failed to parse number"))
			.collect::<Vec<_>>();

		Day01 { nums }
	}
}

impl Day01 {
	pub fn new(nums: Vec<i64>) -> Day01 {
		Day01 { nums }
	}

	pub fn frequency(&self) -> i64 {
		self.nums.iter().fold(0, |y, x| y + x)
	}

	pub fn first_repeat(&self) -> i64 {
		let nums = &self.nums;
		let mut visited = HashSet::new();

		visited.insert(0);

		(0..)
			.map(|i| nums[i % nums.len()])
			.scan(0, |y, x| {
				*y += x;

				Some(*y)
			})
			.find(|&n| visited.replace(n).is_some())
			.expect("Did not find a repeat")
	}
}

fn main() {
	let day: Day01 = io::stdin().lock().into();

	println!("Frequency: {}", day.frequency());
	println!("First Repeat: {}", day.first_repeat());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse() {
		let day: Day01 = "+1\n-2\n+3".as_bytes().into();

		assert_eq!(day, Day01::new(vec![1, -2, 3]));
	}

	#[test]
	fn frequency() {
		assert_eq!(Day01::new(vec![1, 1, 1]).frequency(), 3);
		assert_eq!(Day01::new(vec![1, 1, -2]).frequency(), 0);
		assert_eq!(Day01::new(vec![-1, -2, -3]).frequency(), -6);
	}

	#[test]
	fn first_repeat() {
		assert_eq!(Day01::new(vec![1, -1]).first_repeat(), 0);
		assert_eq!(Day01::new(vec![3, 3, 4, -2, -4]).first_repeat(), 10);
		assert_eq!(Day01::new(vec![-6, 3, 8, 5, -6]).first_repeat(), 5);
		assert_eq!(Day01::new(vec![7, 7, -2, -7, -4]).first_repeat(), 14);
	}
}
