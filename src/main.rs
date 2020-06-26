use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::env;
use std::process;
use std::process::exit;
use std::io::LineWriter;
use miller_rabin::is_prime;



fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 {
		println!("Not enough args, you atleast need to put input and output file names");
		process::exit(1);
	}
	let file_input_name = &args[1];
    let file_output_name = &args[2];
 
	//let mut file = File::create(file_output_name).unwrap();
	//file.write_all(b"5 \n10")?;
	
	let x = 8;
	println!("test : {}",test_prime(&x));
	println!("Test succeed");
	
	let raw_input = fs::read_to_string(file_input_name).expect("Error reading file");
	let input: Vec<i32> = raw_input.trim().split("\r\n").map(|x| x.parse().unwrap()).collect();
	
	let file  = File::create(file_output_name).unwrap();
	let file = LineWriter::new(file);
	
	let mut factorised : Vec<Vec<i32> > = vec![];
	for number in input {
		factorised.push(factorize(number));
	}
	output(factorised, file).unwrap();
	
	
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

fn gcd(mut a:i32,mut b: i32) -> i32 {
	if a < 1 || b < 1 {
		1
	} else {

	let mut r = 0;
	
	while b != 0 {
		r = a % b;
		a = b;
		b = r;
	}
	a
	}
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


fn output(vec : Vec<Vec<i32> >, mut file : LineWriter<File>) -> std::io::Result<()> {
	for subvec in vec {
		for i in 0..subvec.len() {
			let string = if i > 0 {" ".to_string() + &subvec[i].to_string()} else {subvec[i].to_string()};
			file.write_all(string.as_bytes())?;
		}
		file.write_all(b"\r\n")?;
	}
	Ok(())
}
