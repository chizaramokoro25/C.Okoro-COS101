use std::io;

fn help_you_solve_a_problem() {
	let aot = "Area of a Trapezium";
	let aor = "Area of a Rhombus";
	let p = "Area of parellogram";
	let aoc = "Area of a Cube";
	let voc = "Volume of Cylinder";
	println!("{}, \n{} , \n{}, \n{}, \n{}", aot, aor, p, aoc, voc);
    
    println!("Choose an equation:");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("oops, not a valid input");
	let input = input.trim(); 
	if input == aot {
		println!("Enter value of a(length): ");
		let mut input1 = String::new();
		io::stdin().read_line(&mut input1).expect("oops, not a valid input");
		let a:f32 = input1.trim().parse().expect("invalid input");

		println!("Enter value of b(length): ");
		let mut input2 = String::new();
		io::stdin().read_line(&mut input2).expect("oops, not a valid input");
		let b:f32 = input2.trim().parse().expect("invalid input");

		println!("Enter value of height: ");
		let mut input3 = String::new();
		io::stdin().read_line(&mut input3).expect("oops, not a valid input");
		let h:f32 = input3.trim().parse().expect("invalid input");
		
		let area_of_a_trapezium = 0.5 * ((a + b) * h);
		println!("Area of a Trapezium: {}", area_of_a_trapezium); 
	}
	else if input == aor {
		println!("Enter value of d1 (length):");
		let mut d1 = String::new();
		io::stdin().read_line(&mut d1).expect("oops, not a valid input");
		let d1:f32 = d1.trim().parse().expect("invalid input");

		println!("Enter value of d2 (length)");
		let mut d2 = String::new();
		io::stdin().read_line(&mut d2).expect("oops, not a valid input");
		let d2:f32 = d2.trim().parse().expect("invalid input");

		let area_of_a_rhombus = 0.5 * d1 * d2;
		println!("Area of a Rhombus: {}", area_of_a_rhombus);
	}
	else if input == p { 
		println!("Enter the base length:");
		let mut bl = String::new();
		io::stdin().read_line(&mut bl).expect("oops, not a valid input");
		let bl:f32 = bl.trim().parse().expect("invalid input");

		println!("Enter the height:");
		let mut ph = String::new();
		io::stdin().read_line(&mut ph).expect("oops, not a valid input");
		let ph:f32 = ph.trim().parse().expect("invalid input");
		
		let area_of_parellogram = bl * ph;
		println!("Area of a Parallelogram: {}", area_of_parellogram);
	}
	else if input == aoc {
		println!("Enter the length of the side:");
		let mut cube = String::new();
		io::stdin().read_line(&mut cube).expect("oops, not a valid input");
		let cube:f32 = cube.trim().parse().expect("invalid input");
		
		let area_of_a_cube = 6.0 * cube.powf(2.0);
		println!("Area of a cube: {}", area_of_a_cube);
	}
	else if input == voc {
		println!("Radius:");
		let mut r = String::new();
		io::stdin().read_line(&mut r).expect("oops, not a valid input");
		let r:f32 = r.trim().parse().expect("invalid input");

		println!("Enter height:");
		let mut hov = String::new();
		io::stdin().read_line(&mut hov).expect("oops, not a valid input");
		let hov:f32 = hov.trim().parse().expect("invalid input");
		
		let volume_of_a_cylinder = (22.0/7.0) * r.powf(2.0) * hov;
		println!("Volume of a cylinder: {}", volume_of_a_cylinder);
	}
	else {
		println!("Not found, try again");
	}
}

fn main() {
	loop {
	//calling function
	    println!("Hi, this is a Rust program to help you 
    	calculate the area and volume of shapes.");
    	help_you_solve_a_problem();
    	println!("Do you want to input another formula?(yes/no)");
    	let mut again = String::new();
    	io::stdin().read_line(&mut again).expect("invalid input");
    	let again = again.trim().to_lowercase();
    	if again != "yes" {
    		break;
    	}
    }
}
