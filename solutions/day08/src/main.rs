use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Node {
	metadata: Vec<usize>,
	children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
struct Day08 {
	root: Node,
}

impl Day08 {
	fn parse_node(nums: &mut impl Iterator<Item = usize>) -> Node {
		let num_children = nums.next().expect("Expected number of children");
		let num_metadata = nums.next().expect("Expected number of metadata");

		let children =
			(0..num_children).map(|_| Self::parse_node(nums)).collect();

		let metadata = nums.take(num_metadata).collect();

		Node { metadata, children }
	}

	fn new(input: &str) -> Self {
		let mut nums = input
			.split_whitespace()
			.map(|n| n.parse::<usize>())
			.map(|n| n.expect("Failed to parse number"));

		Self {
			root: Self::parse_node(&mut nums),
		}
	}

	fn checksum(&self) -> usize {
		fn sum_metadata(acc: usize, node: &Node) -> usize {
			let sum = acc + node.metadata.iter().sum::<usize>();

			node.children.iter().fold(sum, sum_metadata)
		}

		sum_metadata(0, &self.root)
	}

	fn root_value(&self) -> usize {
		fn sum_nodes(acc: usize, node: &Node) -> usize {
			if node.children.len() == 0 {
				return acc + node.metadata.iter().sum::<usize>();
			}

			node.metadata
				.iter()
				.filter(|&&n| n <= node.children.len())
				.map(|&n| &node.children[n - 1])
				.fold(acc, sum_nodes)
		}

		sum_nodes(0, &self.root)
	}
}

impl<R> From<R> for Day08
where
	R: BufRead,
{
	fn from(mut reader: R) -> Self {
		let mut buf = String::new();

		reader
			.read_to_string(&mut buf)
			.expect("Failed to read input");

		Self::new(&buf)
	}
}

fn main() {
	let day: Day08 = io::stdin().lock().into();

	println!("Checksum: {}", day.checksum());
	println!("Root Value: {}", day.root_value());
}

#[cfg(test)]
mod tests {
	use super::*;

	static TEST_INPUT: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

	#[test]
	fn checksum() {
		let day: Day08 = TEST_INPUT.as_bytes().into();

		assert_eq!(day.checksum(), 138);
	}

	#[test]
	fn root_value() {
		let day: Day08 = TEST_INPUT.as_bytes().into();

		assert_eq!(day.root_value(), 66);
	}
}
