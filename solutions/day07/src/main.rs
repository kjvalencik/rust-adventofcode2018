use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Id = String;

#[derive(Debug, PartialEq)]
struct Day07 {
	dependencies: HashMap<Id, HashSet<Id>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Worker {
	id: Option<String>,
	time: u32,
}

impl Worker {
	fn new() -> Self {
		Self { id: None, time: 0 }
	}
}

impl Day07 {
	fn new(dependencies: HashMap<Id, HashSet<Id>>) -> Self {
		Self { dependencies }
	}

	fn get_extra_time(a: &str) -> u32 {
		(a.as_bytes()[0] - 64) as u32
	}

	fn single_order(&self) -> String {
		let mut items = self.dependencies.clone();
		let mut order = Vec::with_capacity(items.len());

		while items.len() > 0 {
			let mut ready = items
				.iter()
				.filter(|(_, v)| v.len() == 0)
				.map(|(id, _)| id.to_owned())
				.collect::<Vec<_>>();

			ready.sort();

			let next = ready.iter().next().expect("Circular reference");

			items.iter_mut().for_each(|(_, v)| {
				v.remove(next);
			});

			items.remove(next);
			order.push(next.to_owned());
		}

		order.concat()
	}

	fn multi_worker(&self, base_time: u32, num_workers: usize) -> u32 {
		let mut items = self.dependencies.clone();
		let mut workers = vec![Worker::new(); num_workers];
		let mut remaining = items.len();
		let mut total = 0;

		while remaining > 0 {
			let mut should_advance = true;

			// Try to find a free worker
			if let Some(worker) = workers.iter_mut().find(|w| w.id.is_none()) {
				// Try to find available work
				let mut ready = items
					.iter()
					.filter(|(_, v)| v.len() == 0)
					.map(|(id, _)| id.to_owned())
					.collect::<Vec<_>>();

				ready.sort();

				// Try to find work
				if let Some(next) = ready.iter().next() {
					// We might have more free workers, check before advancing
					should_advance = false;

					worker.id = Some(next.to_owned());
					worker.time = base_time + Self::get_extra_time(next);
					items.remove(next);
				}
			}

			// Advance the clock
			if should_advance {
				let time = workers
					.iter()
					.filter(|w| w.id.is_some())
					.map(|w| w.time)
					.min()
					.unwrap_or(0);

				total += time;

				workers.iter_mut().for_each(|w| {
					if let Some(id) = &w.id {
						w.time -= time;

						if w.time == 0 {
							items.iter_mut().for_each(|(_, v)| {
								v.remove(id);
							});

							w.id = None;
							remaining -= 1;
						}
					}
				});
			}
		}

		total
	}
}

impl<R> From<R> for Day07
where
	R: BufRead,
{
	fn from(reader: R) -> Self {
		let pairs = reader
			.lines()
			.map(|line| line.expect("Failed to read line"))
			.map(|line| line.trim().to_owned())
			.filter(|line| line.len() > 0)
			.map(move |line| (line[5..6].to_owned(), line[36..37].to_owned()))
			.collect::<Vec<_>>();

		let mut dependencies =
			pairs
				.iter()
				.fold(HashMap::new(), |mut acc, (dependency, id)| {
					acc.entry(id.to_owned())
						.or_insert_with(|| HashSet::new())
						.insert(dependency.to_owned());

					acc
				});

		pairs.into_iter().for_each(|(id, _)| {
			dependencies.entry(id).or_insert_with(|| HashSet::new());
		});

		Day07::new(dependencies)
	}
}

fn main() {
	let day: Day07 = io::stdin().lock().into();

	println!("Graph Order: {}", day.single_order());
	println!("{}", day.multi_worker(60, 5));
}

#[cfg(test)]
mod tests {
	use super::*;

	static TEST_INPUT: &'static str = r#"
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    "#;

	#[test]
	fn single_order() {
		let day: Day07 = TEST_INPUT.as_bytes().into();

		assert_eq!(day.single_order(), "CABDFE");
	}

	#[test]
	fn multi_worker() {
		let day: Day07 = TEST_INPUT.as_bytes().into();

		assert_eq!(day.multi_worker(0, 2), 15);
	}
}
