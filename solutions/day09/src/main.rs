use std::io::{self, Read};

type BoxError = Box<std::error::Error>;
type Result<T> = std::result::Result<T, BoxError>;

#[derive(Clone, Debug)]
struct Marble {
	prev: usize,
	next: usize,
}

impl Marble {
	fn new() -> Self {
		Self { prev: 0, next: 0 }
	}
}

#[derive(Debug)]
struct Day09 {
	num_players: usize,
	num_marbles: usize,
	next_marble: usize,
	next_player: usize,
	position: usize,
	game: Vec<Marble>,
	scores: Vec<usize>,
}

impl Day09 {
	fn new(num_players: usize, num_marbles: usize) -> Self {
		let day = Day09 {
			num_players,
			num_marbles,
			next_marble: 1,
			next_player: 0,
			position: 0,
			game: vec![Marble::new(); num_marbles + 1],
			scores: vec![0; num_players],
		};

		day.play()
	}

	fn insert_after(&mut self, prev: usize, n: usize) {
		let next = self.game[prev].next;

		self.game[n].prev = prev;
		self.game[n].next = next;
		self.game[prev].next = n;
		self.game[next].prev = n;
	}

	fn remove(&mut self, i: usize) {
		let prev = self.game[i].prev;
		let next = self.game[i].next;

		self.game[prev].next = next;
		self.game[next].prev = prev;
	}

	fn normal_move(&mut self) {
		self.insert_after(self.game[self.position].next, self.next_marble);
		self.position = self.next_marble;
	}

	fn scoring_move(&mut self) {
		let node = (0..7).fold(self.position, |i, _| self.game[i].prev);

		self.scores[self.next_player] += self.next_marble + node;
		self.position = self.game[node].next;
		self.remove(node);
	}

	fn next_move(&mut self) {
		if self.next_marble % 23 == 0 {
			self.scoring_move();
		} else {
			self.normal_move();
		}

		self.next_marble += 1;
		self.next_player = (self.next_player + 1) % self.num_players;
	}

	fn play(mut self) -> Self {
		while self.next_marble <= self.num_marbles {
			self.next_move();
		}

		self
	}

	fn max_score(&self) -> usize {
		*self.scores.iter().max().unwrap_or(&0)
	}
}

impl std::str::FromStr for Day09 {
	type Err = BoxError;

	fn from_str(s: &str) -> Result<Self> {
		let mut parts = s.trim().split_whitespace();

		let num_players =
			parts.next().ok_or("Missing number of players")?.parse()?;

		let num_marbles = parts
			.skip(5)
			.next()
			.ok_or("Missing number of marbles")?
			.parse()?;

		Ok(Self::new(num_players, num_marbles))
	}
}

fn main() {
	let mut input = String::new();

	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let day = input.parse::<Day09>().expect("Failed to parse input");
	let big_day = Day09::new(day.num_players, day.num_marbles * 100);

	println!("Max Score: {}", day.max_score());
	println!("Big Day Max Score: {}", big_day.max_score());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse() {
		let day: Day09 =
			"9 players; last marble is worth 25 points".parse().unwrap();

		assert_eq!(day.num_players, 9);
		assert_eq!(day.num_marbles, 25);
	}

	#[test]
	fn max_score() {
		assert_eq!(Day09::new(9, 25).max_score(), 32);
		assert_eq!(Day09::new(10, 1618).max_score(), 8317);
		assert_eq!(Day09::new(13, 7999).max_score(), 146373);
		assert_eq!(Day09::new(17, 1104).max_score(), 2764);
		assert_eq!(Day09::new(21, 6111).max_score(), 54718);
		assert_eq!(Day09::new(30, 5807).max_score(), 37305);
	}
}
