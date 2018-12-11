use std::cmp;
use std::env;

#[derive(Debug)]
struct Day11 {
	serial: i64,
	grid: Vec<Vec<i64>>,
}

impl Day11 {
	fn power_level(serial: i64, x: i64, y: i64) -> i64 {
		// Find the fuel cell's rack ID, which is its X coordinate plus 10.
		let rack_id = x + 10;

		// Begin with a power level of the rack ID times the Y coordinate.
		let mut power_level = rack_id * y;

		// Increase the power level by the value of the grid serial number.
		power_level += serial;

		// Set the power level to itself multiplied by the rack ID.
		power_level *= rack_id;

		// Keep only the hundreds digit of the power level.
		power_level = (power_level / 100) % 10;

		// Subtract 5 from the power level.
		power_level -= 5;

		power_level
	}

	fn new(serial: i64) -> Self {
		let grid = (1..301)
			.map(|x| {
				(1..301).map(|y| Self::power_level(serial, x, y)).collect()
			})
			.collect();

		Self { serial, grid }
	}

	fn cell(&self, x: usize, y: usize) -> i64 {
		self.grid[x - 1][y - 1]
	}

	fn total_power(&self, n: usize, x: usize, y: usize) -> i64 {
		(0..n).fold(0, |acc, i| {
			(0..n).fold(acc, |acc, j| acc + self.cell(x + i, y + j))
		})
	}

	fn largest_power(&self, size: usize) -> ((usize, usize), i64) {
		let n = 300 - size + 2;

		(1..n)
			.flat_map(|x| (1..n).map(move |y| (x, y)))
			.map(|(x, y)| ((x, y), self.total_power(size, x, y)))
			.max_by_key(|(_, total_power)| *total_power)
			.unwrap_or_else(|| ((0, 0), 0))
	}

	fn largest_power_grid(&self) -> ((usize, usize), usize) {
		(1..301)
			.flat_map(|x| (1..301).map(move |y| (x, y)))
			.flat_map(|(x, y)| {
				let max_size = 302 - cmp::max(x, y);
				let mut total = 0;

				(1..max_size).map(move |n| {
					let row = (x..(x + n))
						.fold(0, |acc, x| acc + self.cell(x, y + n - 1));

					let col = ((y + 1)..(y + n))
						.fold(0, |acc, y| acc + self.cell(x + n - 1, y));

					total += row + col;

					((x, y), n, total)
				})
			})
			.max_by_key(|(_, _, total_power)| *total_power)
			.map(|(xy, n, _)| (xy, n))
			.unwrap_or_else(|| ((0, 0), 0))
	}
}

fn main() {
	let serial = env::args()
		.skip(1)
		.next()
		.expect("Missing serial argument")
		.parse()
		.expect("Invalid serial argument");

	let day = Day11::new(serial);

	let ((x, y), _) = day.largest_power(3);

	println!("Grid Size 3: {},{}", x, y);

	let ((x, y), n) = day.largest_power_grid();

	println!("Largest Power Grid: {},{},{}", x, y, n);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn cell() {
		assert_eq!(Day11::new(8).cell(3, 5), 4);
		assert_eq!(Day11::new(57).cell(122, 79), -5);
		assert_eq!(Day11::new(39).cell(217, 196), 0);
		assert_eq!(Day11::new(71).cell(101, 153), 4);
	}

	#[test]
	fn total_power() {
		let day = Day11::new(18);

		assert_eq!(day.total_power(3, 33, 45), 29);
	}

	#[test]
	fn largest_power() {
		assert_eq!(Day11::new(18).largest_power(3).0, (33, 45));
		assert_eq!(Day11::new(42).largest_power(3).0, (21, 61));
		assert_eq!(Day11::new(18).largest_power(16).0, (90, 269));
		assert_eq!(Day11::new(42).largest_power(12).0, (232, 251));
	}

	// Skip test because it requires `--release` to run in a reasonable amount
	// of time.
	#[test]
	#[ignore]
	fn largest_power_grid() {
		assert_eq!(Day11::new(18).largest_power_grid(), ((90, 269), 16));
		assert_eq!(Day11::new(42).largest_power_grid(), ((232, 251), 12));
	}
}
