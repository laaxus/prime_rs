use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::env;
use std::process;
use std::io::LineWriter;
use miller_rabin::is_prime;
use num::integer::gcd;
use std::thread;
use num::bigint::BigInt;
extern crate num;
extern crate crossbeam_channel;

use crossbeam_channel::unbounded;


fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() > 4 {
		let message = String::from("Too many args.
First arg is the input file name, 'input.txt' by default.
Second arg is the output file name, 'output.txt by default.
Third arg is the number of threads, 4 by default.");
		println!("{}",message);
		process::exit(1);
	}
	let file_input_name = if args.len() >= 2 {args[1].to_owned()} else {String::from("input.txt")};
    let file_output_name = if args.len() >= 3 {args[2].to_owned()} else {String::from("output.txt")};
	let nb_threads = if args.len() >= 4 {args[3].to_owned().parse().expect("Error parsing third arg, number of threads")} else {1};
	
	let (sender_input, receiver_factorize) = unbounded();
	let (sender_factorize,receiver_output) = unbounded();


	// Read Input
	thread::spawn(move || {
		let raw_input = fs::read_to_string(file_input_name).expect("Error reading file");
		let input: Vec<BigInt> = raw_input.trim().split("\r\n").map(|x| x.parse::<BigInt>().expect("Error parsing input")).collect();
		for number in input {
			sender_input.send(number).expect("Error sending to factorize from input");
		}
    });
	
	// factorize 
	for _i in 0..nb_threads {
		let receiver_factorize_clone = receiver_factorize.clone();
		let sender_factorize_clone = sender_factorize.clone();
		thread::spawn(move || {
			while let Ok(val) = receiver_factorize_clone.recv() {
				let factors:Vec<BigInt> = factorize(val);
				sender_factorize_clone.send(factors).expect("Error sending to output from factorize");
			}
		});
	}
	drop(sender_factorize); // Drop original sender so output thread can end peacefully
	
	// output 
	let thread = thread::spawn(move || {
		let file  = File::create(file_output_name).expect("Error creating output file");
		let mut file = LineWriter::new(file);
		while let Ok(factors) = receiver_output.recv() {
			for i in 0..factors.len() {
				let string = if i > 0 {" ".to_string() + &factors[i].to_string()} else {factors[i].to_string()};
				file.write_all(string.as_bytes()).expect("Error writing factors in output file.");
			}
			file.write_all(b"\r\n").expect("Error writing new line in output file.");
		}
	});
	thread.join().expect("Error joining output thread");

}


fn factorize( number : BigInt) -> Vec<BigInt> {

	let mut vec : Vec<BigInt> = vec![number.clone(),BigInt::from(0)];
	
	if number >= BigInt::from(2) && !test_prime(number.clone()) {
		let mut prime_factors : Vec<BigInt> = vec![];
		let mut tuple = (number,BigInt::from(1));
		
		while !test_prime(tuple.0.clone()) {
			tuple = get_factor(tuple.0,BigInt::from(1)); 
			prime_factors.push(tuple.1);
		}
		prime_factors.push(tuple.0);
		
		
		//sort
		prime_factors.sort();
		
		//add no duplicate
		vec[1]+= &BigInt::from(1);
		vec.push(prime_factors[0].clone());
		for i in 1..prime_factors.len() {
			if prime_factors[i] != prime_factors[i-1] {
				vec[1]+=&BigInt::from(1);
				vec.push(prime_factors[i].clone());
			}
		}
	} else {
		vec[1]+= &BigInt::from(1);
		vec.push(number);
	}
	vec
}

fn get_factor(number : BigInt, c:BigInt) -> (BigInt,BigInt) {
	if number.clone() % 2 == BigInt::from(0) {
		(number.clone()/2,BigInt::from(2))
	} else {
		let mut a = BigInt::from(2);
		let mut b = BigInt::from(2);
		let mut d = BigInt::from(1);
		
		while d == BigInt::from(1) {
			a = pollard_rho_f(a.clone(), number.clone(), c.clone());
			
			b = pollard_rho_f(b.clone(), number.clone(), c.clone());
			b = pollard_rho_f(b.clone(), number.clone(), c.clone());
			
			d = gcd(a.clone()-b.clone(),number.clone());
		}
		if d == number {
			
			get_factor(number.clone(),c.clone() + &BigInt::from(1))
		} else {
			(number/d.clone(),d)
		}
	}
	
}

fn pollard_rho_f(x: BigInt, number: BigInt, c: BigInt) -> BigInt {
	(x.clone()*&x + &c) % &number
}


fn test_prime(x: BigInt) -> bool {
	if x == BigInt::from(2) {
		true
	} else if x.clone() % 2 == BigInt::from(0) {
		false
	} else {
		is_prime(&x, 100)
	}
}



