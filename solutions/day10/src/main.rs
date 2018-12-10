use std::io::{self, Read};
use std::str::FromStr;

type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct XY(i64, i64);

impl FromStr for XY {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut parts = s.split(|c| c == ',').map(|n| n.trim().parse());

		let x = parts.next().ok_or("Missing X")??;
		let y = parts.next().ok_or("Missing Y")??;

		Ok(Self(x, y))
	}
}

#[derive(Debug)]
struct Position {
	x: i64,
	y: i64,
}

#[derive(Debug)]
struct Velocity {
	x: i64,
	y: i64,
}

#[derive(Debug)]
struct Point {
	position: Position,
	velocity: Velocity,
}

impl Point {
	fn advance(&self, time: i64) -> Position {
		Position {
			x: self.position.x + time * self.velocity.x,
			y: self.position.y + time * self.velocity.y,
		}
	}
}

impl FromStr for Point {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut parts = s.split(|c| c == '<' || c == '>').skip(1);

		let position: XY = parts
			.next()
			.map(|part| part.parse())
			.ok_or("Missing position")??;

		let velocity: XY = parts
			.skip(1)
			.next()
			.map(|part| part.parse())
			.ok_or("Missing position")??;

		Ok(Self {
			position: Position {
				x: position.0,
				y: position.1,
			},
			velocity: Velocity {
				x: velocity.0,
				y: velocity.1,
			},
		})
	}
}

#[derive(Debug)]
struct Day10 {
	points: Vec<Point>,
}

impl Day10 {
	fn new(points: Vec<Point>) -> Self {
		Self { points }
	}

	fn advance(&self, time: i64) -> Vec<Position> {
		self.points
			.iter()
			.map(|point| point.advance(time))
			.collect()
	}

	fn graph(&self, time: i64) -> String {
		let border = 4;
		let points = self.advance(time);

		let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
		let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);

		let points = points
			.iter()
			.map(|p| Position {
				x: p.x - min_x,
				y: p.y - min_y,
			})
			.collect::<Vec<_>>();

		let max_x =
			points.iter().map(|p| p.x).max().unwrap_or(0) + 2 * border + 1;
		let max_y =
			points.iter().map(|p| p.y).max().unwrap_or(0) + 2 * border + 1;
		let mut grid = vec![vec!["."; max_x as usize]; max_y as usize];

		points.iter().for_each(|p| {
			grid[(p.y + border) as usize][(p.x + border) as usize] = "#";
		});

		let lines = grid
			.into_iter()
			.map(|line| line.concat())
			.collect::<Vec<_>>();

		lines.join("\n")
	}

	fn guess(&self) -> i64 {
		let mut prev_width = 0;

		for i in 0.. {
			let points = self.advance(i);
			let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
			let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);
			let width = max_x - min_x;

			if prev_width > 0 && width > prev_width {
				return (i - 1) as i64;
			}

			prev_width = width;
		}

		unreachable!("Should have returned");
	}
}

impl FromStr for Day10 {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let points = s
			.trim()
			.lines()
			.map(|line| line.trim())
			.filter(|line| line.len() > 0)
			.map(|line| line.parse())
			.collect::<Result<Vec<_>>>()?;

		Ok(Self::new(points))
	}
}

fn main() {
	let mut input = String::new();

	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let day = input.parse::<Day10>().expect("Failed to parse input");
	let guess = day.guess();

	println!("After {} seconds:", guess);
	println!("{}", day.graph(guess));
}

#[cfg(test)]
mod tests {
	use super::*;

	static TEST_INPUT: &'static str = r#"
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>
    "#;

	#[test]
	fn guess() {
		let day: Day10 = TEST_INPUT.parse().unwrap();

		assert_eq!(day.guess(), 3);
	}

	#[test]
	fn graph() {
		let day: Day10 = TEST_INPUT.parse().unwrap();

		assert_eq!(
			day.graph(3),
			r#"
..................
..................
..................
..................
....#...#..###....
....#...#...#.....
....#...#...#.....
....#####...#.....
....#...#...#.....
....#...#...#.....
....#...#...#.....
....#...#..###....
..................
..................
..................
..................
        "#
			.trim()
		);
	}
}
