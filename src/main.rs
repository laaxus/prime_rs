use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::env;
use std::process;
use std::io::LineWriter;
use miller_rabin::is_prime;
use num::integer::gcd;
use std::thread;

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
		let input: Vec<i32> = raw_input.trim().split("\r\n").map(|x| x.parse().expect("Error parsing input")).collect();
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
				let factors:Vec<i32> = factorize(val);
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


fn factorize( number : i32) -> Vec<i32> {

	let mut vec : Vec<i32> = vec![number,0];
	
	if number >= 2 && !test_prime(&number) {
		let mut prime_factors : Vec<i32> = vec![];
		let mut tuple = (number,1);
		
		while !test_prime(&tuple.0) {
			tuple = get_factor(tuple.0,1); 
			prime_factors.push(tuple.1);
		}
		prime_factors.push(tuple.0);
		
		
		//sort
		prime_factors.sort();
		
		//add no duplicate
		vec[1]+=1;
		vec.push(prime_factors[0]);
		for i in 1..prime_factors.len() {
			if prime_factors[i] != prime_factors[i-1] {
				vec[1]+=1;
				vec.push(prime_factors[i]);
			}
		}
	} else {
		vec[1]+=1;
		vec.push(number);
	}
	vec
}

fn get_factor(number : i32, c:i32) -> (i32,i32) {
	if number % 2 == 0 {
		(number/2,2)
	} else {
		let mut a = 2;
		let mut b = 2;
		let mut d = 1;
		
		while d == 1 {
			a = pollard_rho_f(a, number, c);
			
			b = pollard_rho_f(b, number, c);
			b = pollard_rho_f(b, number, c);
			
			d = gcd(a-b,number);
		}
		if d == number {
			
			get_factor(number,c+1)
		} else {
			(number/d,d)
		}
	}
	
}

fn pollard_rho_f(x: i32, number: i32, c: i32) -> i32 {
	(x*x + c) % number
}


fn test_prime(x: &i32) -> bool {
	if *x == 2 {
		true
	} else if *x % 2 == 0 {
		false
	} else {
		is_prime(x, 100)
	}
}



