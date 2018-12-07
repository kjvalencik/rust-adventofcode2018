use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl<S> From<S> for Coordinate
where
	S: AsRef<str>,
{
	fn from(line: S) -> Self {
		let mut parts = line.as_ref().split(|c| c == ',');

		let x = parts
			.next()
			.expect("Missing X")
			.trim()
			.parse::<i32>()
			.expect("Invalid X")
			+ 1;

		let y = parts
			.next()
			.expect("Missing Y")
			.trim()
			.parse::<i32>()
			.expect("Invalid Y")
			+ 1;

		Coordinate { x, y }
	}
}

#[derive(Debug, PartialEq)]
struct Day06 {
	width: usize,
	height: usize,
	coordinates: Vec<Coordinate>,
	grid: Vec<Vec<Vec<i32>>>,
}

#[derive(Debug, PartialEq)]
struct Closest {
	distances: Vec<Vec<Option<usize>>>,
}

impl Closest {
	fn total(&self, n: usize) -> usize {
		let mut total = 0;
		let width = self.distances.len();

		for i in 0..width {
			let row = &self.distances[i];
			let height = row.len();

			for j in 0..height {
				if row[j] == Some(n) {
					if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
						return 0;
					} else {
						total += 1;
					}
				}
			}
		}

		total
	}
}

impl Day06 {
	fn new(coordinates: Vec<Coordinate>) -> Self {
		let width = coordinates
			.iter()
			.map(|point| point.x)
			.max()
			.expect("Expected at least one coordinate") as usize
			+ 2;

		let height = coordinates
			.iter()
			.map(|point| point.x)
			.max()
			.expect("Expected at least one coordinate") as usize
			+ 2;

		let mut grid = vec![vec![vec![0; coordinates.len()]; height]; width];

		// Fill the grid with distances
		for i in 0..width {
			for j in 0..height {
				for k in 0..coordinates.len() {
					let Coordinate { x, y } = coordinates[k];

					grid[i][j][k] = (x - i as i32).abs() + (y - j as i32).abs();
				}
			}
		}

		Self {
			width,
			height,
			coordinates,
			grid,
		}
	}

	fn closest(&self) -> Closest {
		let distances = self
			.grid
			.iter()
			.map(|line| {
				line.iter()
					.map(|distances| {
						distances
							.iter()
							.enumerate()
							.min_by_key(|(_, d)| *d)
							.and_then(|(i, d)| {
								let equal = distances
									.iter()
									.enumerate()
									.find(|(j, d2)| i != *j && d == *d2);

								if equal.is_some() {
									return None;
								}

								Some(i)
							})
					})
					.collect()
			})
			.collect();

		Closest { distances }
	}

	fn largest_area(&self) -> usize {
		let closest = Self::closest(self);

		(0..self.coordinates.len())
			.map(|n| closest.total(n))
			.max()
			.expect("Expected at least one coordinate")
	}

	fn close_region_size(&self, n: i32) -> usize {
		(-n..(n + self.width as i32)).fold(0, |acc, i| {
			(-n..(n + self.height as i32))
				.map(|j| {
					self.coordinates.iter().fold(0, |acc, point| {
						acc + (point.x - i).abs() + (point.y - j).abs()
					})
				})
				.filter(|&m| m < n)
				.count() + acc
		})
	}
}

impl<R> From<R> for Day06
where
	R: BufRead,
{
	fn from(reader: R) -> Self {
		let coordinates = reader
			.lines()
			.map(|line| line.expect("Failed to read line"))
			.map(|line| line.trim().to_owned())
			.filter(|line| line.len() > 0)
			.map(Coordinate::from)
			.collect::<Vec<_>>();

		Self::new(coordinates)
	}
}

fn main() {
	let day: Day06 = io::stdin().lock().into();

	println!("Largest area: {}", day.largest_area());
	println!("Close Region Size: {}", day.close_region_size(10000));
}

#[cfg(test)]
mod tests {
	use super::*;

	static TEST_INPUT: &'static str = r#"
        1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9
    "#;

	#[test]
	fn largest_area() {
		let day: Day06 = TEST_INPUT.as_bytes().into();

		assert_eq!(day.largest_area(), 17);
	}

	#[test]
	fn close_region_size() {
		let day: Day06 = TEST_INPUT.as_bytes().into();

		assert_eq!(day.close_region_size(32), 16);
	}
}
