use std::io::{self, Read};

fn remove_pair(n: usize, s: &str) -> Option<(usize, String)> {
	for i in n..(s.len() - 1) {
		let a = &s[i..(i + 1)];
		let b = &s[(i + 1)..(i + 2)];

		if a != b && a.to_uppercase() == b.to_uppercase() {
			let prefix = &s[..i];
			let postfix = &s[(i + 2)..];
			let result = format!("{}{}", prefix, postfix);

			return Some((i, result));
		}
	}

	None
}

fn reaction(initial: &str) -> String {
	let mut prev = initial.to_owned();
	let mut prev_n = 0;

	while let Some((n, next)) = remove_pair(prev_n, &prev) {
		prev = next;
		prev_n = if n > 0 { n - 1 } else { 0 };
	}

	prev
}

fn improve_reaction(s: &str) -> String {
	let lower = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
	let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();

	(0..lower.len())
		.map(|i| {
			let a = lower[i];
			let b = upper[i];

			s.chars().filter(|&c| c != a && c != b).collect::<String>()
		})
		.map(|s| reaction(&s))
		.min_by_key(|r| r.len())
		.expect("Expected there to be at least one reaction")
}

fn main() {
	let mut buf = String::new();

	io::stdin()
		.read_to_string(&mut buf)
		.expect("Failed to read from stdin");

	let result = reaction(buf.trim());
	let improved = improve_reaction(buf.trim());

	println!("Number of units: {}", result.len());
	println!("Number of improved units: {}", improved.len());
}

#[cfg(test)]
mod tests {
	#[test]
	fn reaction() {
		assert_eq!(super::reaction("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
	}

	#[test]
	fn improve_reaction() {
		assert_eq!(super::improve_reaction("dabAcCaCBAcCcaDA"), "daDA");
	}
}
