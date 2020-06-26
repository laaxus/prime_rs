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
	
	if args.len() < 2 {
		println!("Not enough args, you atleast need to put input and output file names");
		process::exit(1);
	}
	let file_input_name = &args[1].to_string().clone();
    let file_output_name = &args[2].to_string().clone();
 
	
	let (s0, r0) = unbounded();
	let (sn,rn) = unbounded();

	let nbt = 1;

	
	// Read Input
	let raw_input = fs::read_to_string(file_input_name).expect("Error reading file");
	thread::spawn(move || {
		let input: Vec<i32> = raw_input.trim().split("\r\n").map(|x| x.parse().unwrap()).collect();
		for number in input {
			println!("send input : {}",number);
			s0.send(number).expect("Error sending to factorize from input");
		}
    });
	
	// factorize 
	for _i in 0..nbt {
		let r1 = r0.clone();
		let s2 = sn.clone();
		thread::spawn(move || {
			while let Ok(val) = r1.recv() {
				println!("received factorize : {}",val);
				let vec:Vec<i32> = factorize(val);
				s2.send(vec).expect("Error sending to output from factorize");
			}
		});
		
	}
	
	// output 
	let file  = File::create(file_output_name).unwrap();
	let mut file = LineWriter::new(file);
	let thread = thread::spawn(move || {
		while let Ok(subvec) = rn.recv() {
			for i in 0..subvec.len() {
				let string = if i > 0 {" ".to_string() + &subvec[i].to_string()} else {subvec[i].to_string()};
				file.write_all(string.as_bytes()).unwrap();
			}
			file.write_all(b"\r\n").unwrap();
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



