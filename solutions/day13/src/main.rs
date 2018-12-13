use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c = match self {
			Direction::Left => '<',
			Direction::Right => '>',
			Direction::Up => '^',
			Direction::Down => 'v',
		};

		write!(f, "{}", c)
	}
}

#[derive(Debug)]
struct Cart {
	direction: Direction,
	turns: u32,
	visited: bool,
}

impl Cart {
	fn new(direction: Direction) -> Self {
		Self {
			direction,
			turns: 0,
			visited: false,
		}
	}
}

impl fmt::Display for Cart {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.direction)
	}
}

#[derive(Debug)]
enum TrackType {
	CurveForward,
	CurveBackward,
	Horizontal,
	Vertical,
	Intersection,
}

impl fmt::Display for TrackType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c = match self {
			TrackType::CurveForward => '/',
			TrackType::CurveBackward => '\\',
			TrackType::Horizontal => '-',
			TrackType::Vertical => '|',
			TrackType::Intersection => '+',
		};

		write!(f, "{}", c)
	}
}

#[derive(Debug)]
struct Track {
	track_type: TrackType,
	cart: Option<Cart>,
}

impl Track {
	fn new(track_type: TrackType) -> Self {
		Self {
			track_type,
			cart: None,
		}
	}

	fn from_cart(cart: Cart) -> Self {
		let track_type = match cart.direction {
			Direction::Left | Direction::Right => TrackType::Horizontal,
			Direction::Up | Direction::Down => TrackType::Vertical,
		};

		Self {
			track_type,
			cart: Some(cart),
		}
	}
}

impl fmt::Display for Track {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(cart) = &self.cart {
			cart.fmt(f)
		} else {
			self.track_type.fmt(f)
		}
	}
}

#[derive(Debug)]
struct Day13 {
	map: Vec<Vec<Option<Track>>>,
	num_carts: usize,
}

impl fmt::Display for Day13 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in self.map.iter() {
			for track in row.iter() {
				if let Some(track) = track {
					track.fmt(f)?;
				} else {
					write!(f, " ")?;
				}
			}

			write!(f, "\n")?;
		}

		Ok(())
	}
}

impl Day13 {
	fn dimensions(&self) -> (usize, usize) {
		let rows = self.map.len();
		let cols = self.map.iter().map(|row| row.len()).max().unwrap_or(0);

		(rows, cols)
	}

	fn tick(&mut self) -> Vec<(usize, usize)> {
		let mut collisions = Vec::new();
		let (rows, cols) = self.dimensions();

		for y in 0..rows {
			for x in 0..cols {
				if y >= self.map.len() || x >= self.map[y].len() {
					continue;
				}

				if let Some(track) = &mut self.map[y][x] {
					if let Some(cart) = &mut track.cart {
						cart.visited = false;
					}
				}
			}
		}

		for y in 0..rows {
			for x in 0..cols {
				if y >= self.map.len() || x >= self.map[y].len() {
					continue;
				}

				let res = if let Some(track) = &mut self.map[y][x] {
					if let Some(mut cart) = track.cart.take() {
						if cart.visited {
							track.cart.replace(cart);

							None
						} else {
							let (direction, position) = match track.track_type {
								TrackType::CurveForward => match cart.direction
								{
									Direction::Up => {
										(Direction::Right, (x + 1, y))
									}
									Direction::Down => {
										(Direction::Left, (x - 1, y))
									}
									Direction::Left => {
										(Direction::Down, (x, y + 1))
									}
									Direction::Right => {
										(Direction::Up, (x, y - 1))
									}
								},
								TrackType::CurveBackward => {
									match cart.direction {
										Direction::Up => {
											(Direction::Left, (x - 1, y))
										}
										Direction::Down => {
											(Direction::Right, (x + 1, y))
										}
										Direction::Left => {
											(Direction::Up, (x, y - 1))
										}
										Direction::Right => {
											(Direction::Down, (x, y + 1))
										}
									}
								}
								TrackType::Horizontal => match cart.direction {
									Direction::Left => {
										(Direction::Left, (x - 1, y))
									}
									Direction::Right => {
										(Direction::Right, (x + 1, y))
									}
									_ => panic!("Impossible movement"),
								},
								TrackType::Vertical => match cart.direction {
									Direction::Up => {
										(Direction::Up, (x, y - 1))
									}
									Direction::Down => {
										(Direction::Down, (x, y + 1))
									}
									_ => panic!("Impossible movement"),
								},
								TrackType::Intersection => {
									cart.turns += 1;

									match cart.direction {
										Direction::Up => match cart.turns % 3 {
											1 => (Direction::Left, (x - 1, y)),
											2 => (Direction::Up, (x, y - 1)),
											_ => (Direction::Right, (x + 1, y)),
										},
										Direction::Down => match cart.turns % 3
										{
											1 => (Direction::Right, (x + 1, y)),
											2 => (Direction::Down, (x, y + 1)),
											_ => (Direction::Left, (x - 1, y)),
										},
										Direction::Left => match cart.turns % 3
										{
											1 => (Direction::Down, (x, y + 1)),
											2 => (Direction::Left, (x - 1, y)),
											_ => (Direction::Up, (x, y - 1)),
										},
										Direction::Right => {
											match cart.turns % 3 {
												1 => {
													(Direction::Up, (x, y - 1))
												}
												2 => (
													Direction::Right,
													(x + 1, y),
												),
												_ => (
													Direction::Down,
													(x, y + 1),
												),
											}
										}
									}
								}
							};

							cart.direction = direction;
							cart.visited = true;

							Some((cart, position))
						}
					} else {
						None
					}
				} else {
					None
				};

				if let Some((cart, (i, j))) = res {
					if let Some(track) = &mut self.map[j][i] {
						if track.cart.take().is_some() {
							collisions.push((i, j));
						} else {
							track.cart = Some(cart);
						}
					} else {
						panic!("Tried to move cart off track");
					}
				}
			}
		}

		self.num_carts -= 2 * collisions.len();

		collisions
	}

	fn simulate(&mut self) -> (usize, usize) {
		while self.num_carts > 1 {
			self.tick();
		}

		let (rows, cols) = self.dimensions();

		for y in 0..rows {
			for x in 0..cols {
				if y >= self.map.len() || x >= self.map[y].len() {
					continue;
				}

				if let Some(track) = &self.map[y][x] {
					if track.cart.is_some() {
						return (x, y);
					}
				}
			}
		}

		unreachable!("Should have at least one car left");
	}
}

impl FromStr for Day13 {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let map = s
			.lines()
			.map(|line| {
				line.chars().map(|c| {
					Ok(match c {
						'/' => Ok(Some(Track::new(TrackType::CurveForward))),
						'\\' => Ok(Some(Track::new(TrackType::CurveBackward))),
						'-' => Ok(Some(Track::new(TrackType::Horizontal))),
						'|' => Ok(Some(Track::new(TrackType::Vertical))),
						'+' => Ok(Some(Track::new(TrackType::Intersection))),
						' ' => Ok(None),
						'<' => Ok(Some(Track::from_cart(Cart::new(
							Direction::Left,
						)))),
						'>' => Ok(Some(Track::from_cart(Cart::new(
							Direction::Right,
						)))),
						'^' => {
							Ok(Some(Track::from_cart(Cart::new(Direction::Up))))
						}
						'v' => Ok(Some(Track::from_cart(Cart::new(
							Direction::Down,
						)))),
						_ => Err(format!("Unexpected map character: {}", c)),
					}?)
				})
			})
			.map(|line| line.collect::<Result<Vec<_>>>())
			.collect::<Result<Vec<_>>>()?;

		let num_carts = map.iter().fold(0, |acc, line| {
			line.iter()
				.map(|track| {
					if let Some(track) = track {
						if track.cart.is_some() {
							return 1;
						}
					}

					0
				})
				.fold(acc, |y, x| y + x)
		});

		Ok(Self { map, num_carts })
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

	let mut day: Day13 = input.parse().expect("Failed to parse input");

	loop {
		let collisions = day.tick();
		if collisions.len() > 0 {
			let (x, y) = collisions[0];

			println!("First collision: {},{}", x, y);
			break;
		}
	}

	let (x, y) = day.simulate();

	println!("Last car: {},{}", x, y);
}

#[cfg(test)]
mod tests {
	use super::*;

	static COLLISION_INPUT: &'static str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#;

	static LAST_CAR_INPUT: &'static str = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

	#[test]
	fn collision() {
		let mut day: Day13 = COLLISION_INPUT.parse().unwrap();

		loop {
			let collisions = day.tick();
			if collisions.len() > 0 {
				let collision = collisions[0];

				assert_eq!(collision, (7, 3));
				break;
			}
		}
	}

	#[test]
	fn simulate() {
		let mut day: Day13 = LAST_CAR_INPUT.parse().unwrap();
		let position = day.simulate();

		assert_eq!(position, (6, 4));
	}
}
