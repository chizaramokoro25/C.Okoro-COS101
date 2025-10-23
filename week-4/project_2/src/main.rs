// Rust program to input the age and experience 
// of an employee to determine annual incentive
use std::io;

fn main() {
	loop {
		println!("Annual Incentive");
		// input name 
		println!("Type in name of entity ");
		let mut name = String::new();
		io::stdin().read_line(&mut name).expect("oops, failed to read input");
		let name = name.trim();

		// input age
		println!("Type in age of entity ");
		let mut age = String::new();
		io::stdin().read_line(&mut age).expect("oops, failed to read input");
		let age:f32 = age.trim().parse().expect("invalid input");

		// input experience
		println!("Are they experienced? (yes/no)");
		let mut experience = String::new();
		io::stdin().read_line(&mut experience).expect("oops, failed to read input");
		let experience = experience.trim().to_lowercase();

		println!("Name: {}", name);
		println!("Age: {}", age);
		println!("Experience: {}", experience);

		// assign annual incentive
		if experience == "no" {
			println!("Annual Incentive: 100000");
		} else if experience == "yes" {
			if age >= 40.0 {
				let incentive1 = 1_560_000.00;
				println!("Annual Incentive: {}", incentive1);
			} else if age >= 30.0 && age < 40.0 {
				let incentive2 = 1_480_000.00;
				println!("Annual Incentive: {}", incentive2);
			} else if age > 30.0 {
				let incentive3 = 1_300_000.00;
				println!("Annual Incentive: {}", incentive3);
			}
		}    
		else {
			println!("Invalid input. Please type yes/no");
			continue;
		}
		println!("Do you want to enter another record? (yes/no)");
		let mut again = String::new();
		io::stdin().read_line(&mut again).expect("Please, enter a valid input");
		let again = again.trim().to_lowercase();
		if again != "yes"{
			break;
		}	
	}
}
