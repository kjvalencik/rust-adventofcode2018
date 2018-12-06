use std::collections::HashSet;
use std::io::{self, BufRead};

use chrono::offset::TimeZone;
use chrono::{DateTime, Duration, Timelike, Utc};

type Id = usize;

#[derive(Debug, PartialEq)]
enum Event {
	Shift(Id),
	Asleep,
	Wake,
}

#[derive(Debug, PartialEq)]
struct Log {
	timestamp: DateTime<Utc>,
	event: Event,
}

impl<S> From<S> for Log
where
	S: AsRef<str>,
{
	// [1518-11-01 00:00] Guard #10 begins shift
	fn from(line: S) -> Self {
		let mut parts = line.as_ref().split(']');
		let timestamp = parts.next().expect("Missing timestamp");
		let line = &parts.next().expect("Missing log entry")[1..];

		let event = match &line[0..1] {
			"G" => {
				let id = &line
					.split(' ')
					.skip(1)
					.next()
					.expect("Missing identifier")[1..]
					.parse()
					.expect("Invalid identifier");

				Event::Shift(*id)
			}
			"f" => Event::Asleep,
			"w" => Event::Wake,
			_ => panic!("Unexpected event"),
		};

		Log {
			event,
			timestamp: Utc
				.datetime_from_str(&timestamp[1..], "%Y-%m-%d %H:%M")
				.expect("Invalid date format"),
		}
	}
}

#[derive(Debug, PartialEq)]
struct Day04 {
	logs: Vec<Log>,
}

impl Day04 {
	fn new(logs: Vec<Log>) -> Self {
		Day04 { logs }
	}

	fn ids(&self) -> HashSet<Id> {
		self.logs
			.iter()
			.filter_map(|log| match log.event {
				Event::Shift(id) => Some(id),
				_ => None,
			})
			.collect()
	}

	fn shifts<'a>(self: &'a Self, id: Id) -> impl Iterator<Item = &Log> + 'a {
		let mut is_match = false;

		self.logs.iter().filter(move |log| {
			match log.event {
				Event::Shift(shift_id) => is_match = id == shift_id,
				_ => {}
			};

			is_match
		})
	}

	// This only works because awake/asleep come in perfect pairs
	fn sleeps<'a>(
		self: &'a Self,
		id: Id,
	) -> impl Iterator<Item = (DateTime<Utc>, DateTime<Utc>)> + 'a {
		let wakes = self
			.shifts(id)
			.filter(|log| log.event == Event::Wake)
			.map(|log| log.timestamp);

		self.shifts(id)
			.filter(|log| log.event == Event::Asleep)
			.map(|log| log.timestamp)
			.zip(wakes)
	}

	fn total_asleep(&self, id: Id) -> i64 {
		self.sleeps(id).fold(0, |acc, (start, end)| {
			let duration = end.signed_duration_since(start).num_milliseconds();

			acc + duration
		})
	}

	fn max_asleep(&self) -> Id {
		self.ids()
			.into_iter()
			.map(|id| (id, self.total_asleep(id)))
			.max_by_key(|(_, total)| *total)
			.map(|(id, _)| id)
			.expect("Expected someone to sleep")
	}

	fn max_minute_asleep(&self, id: Id) -> (usize, u32) {
		let minutes = self
			.sleeps(id)
			.flat_map(|(start, end)| {
				let minute = start.time().minute();

				(0..)
					.take_while(move |i| start + Duration::minutes(*i) < end)
					.map(move |i| (minute as i64 + i) % 60)
			})
			.fold([0; 60], |mut acc, i| {
				acc[i as usize] += 1;

				acc
			});

		minutes
			.iter()
			.enumerate()
			.max_by_key(|(_, n)| *n)
			.map(|(i, n)| (i, *n))
			.expect("Expected at least one sleep")
	}

	fn max_frequency_asleep(&self) -> (Id, usize) {
		let (id, (minute, _)) = self
			.ids()
			.into_iter()
			.map(|id| (id, self.max_minute_asleep(id)))
			.max_by_key(|(_, (_, count))| *count)
			.expect("Expected at least one sleep");

		(id, minute)
	}
}

impl<R> From<R> for Day04
where
	R: BufRead,
{
	fn from(reader: R) -> Self {
		let mut logs = reader
			.lines()
			.map(|line| line.expect("Failed to read line"))
			.map(|line| line.trim().to_owned())
			.filter(|line| line.len() > 0)
			.map(Log::from)
			.collect::<Vec<_>>();

		logs.sort_by_key(|l| l.timestamp);

		Day04::new(logs)
	}
}

fn main() {
	let day: Day04 = io::stdin().lock().into();

	let guard = day.max_asleep();
	let (minute, _) = day.max_minute_asleep(guard);

	println!("Most asleep checksum: {}", guard * minute);

	let (id, minute) = day.max_frequency_asleep();

	println!("Most frequently asleep checksum: {}", id * minute);
}

#[cfg(test)]
mod tests {
	use super::*;

	static TEST_INPUT: &'static str = r#"
		[1518-11-01 00:00] Guard #10 begins shift
		[1518-11-01 00:05] falls asleep
		[1518-11-01 00:25] wakes up
		[1518-11-01 00:30] falls asleep
		[1518-11-01 00:55] wakes up
		[1518-11-01 23:58] Guard #99 begins shift
		[1518-11-02 00:40] falls asleep
		[1518-11-02 00:50] wakes up
		[1518-11-03 00:05] Guard #10 begins shift
		[1518-11-03 00:24] falls asleep
		[1518-11-03 00:29] wakes up
		[1518-11-04 00:02] Guard #99 begins shift
		[1518-11-04 00:36] falls asleep
		[1518-11-04 00:46] wakes up
		[1518-11-05 00:03] Guard #99 begins shift
		[1518-11-05 00:45] falls asleep
		[1518-11-05 00:55] wakes up
	"#;

	#[test]
	fn ids() {
		let day: Day04 = TEST_INPUT.as_bytes().into();
		let mut ids = day.ids().into_iter().collect::<Vec<_>>();

		ids.sort();

		assert_eq!(ids, vec![10, 99]);
	}

	#[test]
	fn max_minute() {
		let day: Day04 = TEST_INPUT.as_bytes().into();
		let guard = day.max_asleep();
		let (minute, count) = day.max_minute_asleep(guard);

		assert_eq!(guard, 10);
		assert_eq!(minute, 24);
		assert_eq!(count, 2);
	}

	#[test]
	fn max_frequency() {
		let day: Day04 = TEST_INPUT.as_bytes().into();
		let (id, minute) = day.max_frequency_asleep();

		assert_eq!(id, 99);
		assert_eq!(minute, 45);
	}
}
