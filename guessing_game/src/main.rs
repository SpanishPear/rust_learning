use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

	println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);	


    loop {
	    print!("Please input your guess: ");	    //https://stackoverflow.com/questions/37531903/how-do-i-print-output-without-a-trailing-newline-in-rust
	    io::stdout().flush().expect("flush failed :(");

		// in rust - variables are immutable default
		// 		ie, constants
		// 		adding `mut` makes them mutable
		// let foo = 5; // immutable
		// let mut bar = 5; // mutable

		// new is as method for String
		// Q: is this the same as ""?
		let mut guess = String::new();

		io::stdin()
			// we need the & for ref
			// we need the mut to make ref mutable
			.read_line(&mut guess)
			.expect("Failed to read line");

		// rust allows shadowing..?
		// we need to say u32 so parse knows what type
	    let guess: u32 = match guess
	    					.trim()  // python strip
	    					.parse() {
	    						Ok(num) => num,
	    						Err(_) => continue
	    					};

	    
		match guess.cmp(&secret_number) {
	        Ordering::Less => println!("Too small!"),
	        Ordering::Greater => println!("Too big!"),
	        Ordering::Equal => {
	        	println!("You win!");
				break;
	        },
		}

    }

}
