use std::env;

#[derive(Debug)]
struct Elf {
	position: usize,
}

impl Elf {
	fn new(position: usize) -> Self {
		Self { position }
	}
}

#[derive(Debug)]
struct Day14 {
	elves: Vec<Elf>,
	recipes: Vec<usize>,
}

impl Day14 {
	fn new() -> Self {
		let recipes = vec![3, 7];
		let elves = recipes
			.iter()
			.enumerate()
			.map(|(i, _)| Elf::new(i))
			.collect();

		Self { recipes, elves }
	}

	fn next_recipes(&mut self) -> usize {
		let total: usize = self
			.elves
			.iter()
			.map(|elf| self.recipes[elf.position])
			.sum();

		let mut n = total;
		let mut digit = 1;
		let recipes = (0..)
			.scan(0, |_, i| {
				if i > 0 && digit > total {
					return None;
				}

				let m = n % 10;

				n /= 10;
				digit *= 10;

				Some(m)
			})
			.collect::<Vec<_>>();

		let added = recipes.len();

		self.recipes.extend(recipes.iter().rev());

		for i in 0..self.elves.len() {
			let mut elf = &mut self.elves[i];
			let score = self.recipes[elf.position];

			elf.position = (elf.position + score + 1) % self.recipes.len()
		}

		added
	}

	fn simulate(&mut self, n: usize) -> &[usize] {
		while self.recipes.len() < n + 10 {
			self.next_recipes();
		}

		&self.recipes[n..(n + 10)]
	}

	fn simulate_two(&mut self, input: &[usize]) -> usize {
		loop {
			let n = self.next_recipes();

			if self.recipes.len() < input.len() + n {
				continue;
			}

			let start = self.recipes.len() - input.len() - n;
			let end = start + n;

			for i in start..end {
				if &self.recipes[i..(i + input.len())] == input {
					return i;
				}
			}
		}
	}
}

fn main() {
	let recipes = env::args()
		.skip(1)
		.next()
		.expect("Missing recipes argument")
		.parse()
		.expect("Invalid recipes argument");

	let mut day = Day14::new();
	let res = day
		.simulate(recipes)
		.into_iter()
		.map(|n| n.to_string())
		.collect::<String>();

	println!("Recipes: {}", res);

	let mut day = Day14::new();
	let recipes = env::args()
		.skip(1)
		.next()
		.expect("Missing recipes argument")
		.chars()
		.map(|c| c.to_string().parse())
		.collect::<Result<Vec<_>, _>>()
		.expect("Invalid recipes argument");

	println!("Previous recipes: {}", day.simulate_two(&recipes));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simulate() {
		assert_eq!(Day14::new().simulate(9), [5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
		assert_eq!(Day14::new().simulate(5), [0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
		assert_eq!(Day14::new().simulate(18), [9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
		assert_eq!(Day14::new().simulate(2018), [5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
	}

	#[test]
	fn simulate_two() {
		assert_eq!(Day14::new().simulate_two(&[5, 1, 5, 8, 9]), 9);
	}
}
