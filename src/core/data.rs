#![allow(dead_code)]

use rand::Rng;

pub fn random_data(amt: u32, len: u32) -> Vec<String> {
	// random length per vec
	let mut v: Vec<String> = Vec::with_capacity(amt as usize);
	let mut rng = rand::thread_rng();

	// generating strings of varying length
	for i in 0..amt {
        // creating the char from the iterator
		let char = char::from_u32(65 + i % 26).unwrap();

        // creating `n`
		let ll: u32 = rng.gen::<u32>() % len + 1;

        // forming a string of length n
		v.push((0..ll).map(|_| char).collect::<String>());
	}

	return v;
}

