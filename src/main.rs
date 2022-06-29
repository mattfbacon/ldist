use std::collections::BTreeMap;
use std::io::{stdin, Read};
use std::ascii::AsciiExt as _;

fn main() {
	let mut counts: BTreeMap<char, usize> = BTreeMap::default();
	let stdin = stdin().lock();
	let mut reader = utf8_read::Reader::new(stdin);
	for ch in &mut reader {
		let ch = ch.expect("Invalid UTF-8");
		let ch = ch.to_ascii_lowercase();
		if ch.is_control() || ch == ' ' { continue; }
		*counts.entry(ch).or_default() += 1;
	}
	println!("{:#?}", counts);

	let max_within_letters = counts.iter().map(|(&_ch, &count)| count).max().unwrap() as f64;
	const BAR_SIZE: f64 = 50.0;
	for (&ch, &count) in counts.iter() {
		print!("{ch:2} ({count:4})| ");
		let this_bar_size = (count as f64 / max_within_letters) * BAR_SIZE;
		for _ in 0..(this_bar_size.round() as usize) {
			print!("=");
		}
		println!();
	}
}
