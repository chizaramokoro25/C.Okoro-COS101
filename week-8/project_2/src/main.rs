use std::io;
fn lets_find_them() {
	let mut candidates: Vec<(String, u32)> = Vec::new();
	loop {
		let mut name = String::new();
		let mut years_str = String::new();

		println!("Enter candidate name (or type 'done' to finish):");
		io::stdin().read_line(&mut name).expect("oops, invalid string");
		let name = name.trim();

		if name.eq_ignore_ascii_case("done") {
			break;
		}
  
		println!("Enter years of programming experience:");
		io::stdin().read_line(&mut years_str).expect("oops, invalid string");

		let years:u32 = match years_str.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid number. Try again.");
				continue;
			}
		};
		candidates.push((name.to_string(), years));
	}
	if candidates.is_empty() {
		println!("No candidates entered.");
		return;
	}
	let mut best = &candidates[0];

	for candidate in &candidates {
		if candidate.1 > best.1 {
			best = candidate;
		}
	}

	println!("\nMost experienced candidate: {} ({} years)", best.0, best.1);
}

fn main() {
    println!("Please, enter the following information");
    loop{
    	lets_find_them();
    	print!("Do you want to enter another record? (yes/no)");
    	let mut again = String::new();
    	io::stdin().read_line(&mut again).expect("oops, invalid string");
    	let again = again.trim().to_lowercase();
    	if again != "yes" {
    		break;
    	}
    }
}
