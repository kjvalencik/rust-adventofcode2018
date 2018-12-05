use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Claim {
	id: usize,
	l: usize,
	t: usize,
	w: usize,
	h: usize,
}

impl<S> From<S> for Claim
where
	S: AsRef<str>,
{
	fn from(line: S) -> Self {
		// #1 @ 861,330: 20x10
		let nums = line
			.as_ref()
			.split(|c| !char::is_numeric(c))
			.map(|p| p.parse::<usize>())
			.filter_map(|p| p.ok())
			.collect::<Vec<_>>();

		Claim {
			id: nums[0],
			l: nums[1],
			t: nums[2],
			w: nums[3],
			h: nums[4],
		}
	}
}

#[derive(Debug, PartialEq)]
struct Day03 {
	claims: Vec<Claim>,
	fabric: Vec<Vec<usize>>,
}

impl Day03 {
	fn fabric_dimensions(claims: &Vec<Claim>) -> (usize, usize) {
		let width = claims
			.iter()
			.map(|&Claim { l, w, .. }| l + w)
			.max()
			.expect("Expected at least one claim");

		let height = claims
			.iter()
			.map(|&Claim { t, h, .. }| t + h)
			.max()
			.expect("Expected at least one claim");

		(width, height)
	}

	fn total_overlapped(&self) -> usize {
		self.fabric.iter().fold(0, |acc, line| {
			line.iter().fold(acc, |acc, &x| acc + (x > 1) as usize)
		})
	}

	fn non_overlapped<'a>(
		self: &'a Self,
	) -> impl Iterator<Item = &'a Claim> + 'a {
		self.claims
			.iter()
			.filter(move |&&Claim { id: _, l, t, w, h }| {
				for x in l..(l + w) {
					for y in t..(t + h) {
						if self.fabric[x][y] > 1 {
							return false;
						}
					}
				}

				true
			})
	}
}

impl<R> From<R> for Day03
where
	R: BufRead,
{
	fn from(reader: R) -> Self {
		let claims = reader
			.lines()
			.map(|line| line.expect("Failed to read line"))
			.map(|line| line.trim().to_owned())
			.filter(|line| line.len() > 0)
			.map(Claim::from)
			.collect::<Vec<_>>();

		let (width, height) = Day03::fabric_dimensions(&claims);

		let fabric = claims.iter().fold(
			vec![vec![0; height]; width],
			|mut f, &Claim { id: _, l, t, w, h }| {
				for x in l..(l + w) {
					for y in t..(t + h) {
						f[x][y] += 1;
					}
				}

				f
			},
		);

		Day03 { claims, fabric }
	}
}

fn main() {
	let day: Day03 = io::stdin().lock().into();

	println!("Overlapped: {}", day.total_overlapped());
	day.non_overlapped()
		.for_each(|Claim { id, .. }| println!("Non-overlapped: {}", id));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn total_overlapped() {
		let day: Day03 = r#"
			#1 @ 1,3: 4x4
			#2 @ 3,1: 4x4
			#3 @ 5,5: 2x2
		"#
		.as_bytes()
		.into();

		assert_eq!(day.total_overlapped(), 4);
	}

	#[test]
	fn non_overlapped() {
		let day: Day03 = r#"
			#1 @ 1,3: 4x4
			#2 @ 3,1: 4x4
			#3 @ 5,5: 2x2
		"#
		.as_bytes()
		.into();

		let non_overlapped = day
			.non_overlapped()
			.map(|Claim { id, .. }| *id)
			.collect::<Vec<_>>();

		assert_eq!(non_overlapped, vec![3 as usize]);
	}
}
