// Rust program to find the roots of a quadratic equation
use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first value: ");
    io::stdin().read_line(&mut input1).expect("oops, not a valid string");
    let a:f32 = input1.trim().parse().expect("sorry, not a valid number");

    println!("Enter second value: ");
    io::stdin().read_line(&mut input2).expect("oops, not a valid string");
    let b:f32 = input2.trim().parse().expect("sorry, not a valid number");

    println!("Enter third value: ");
    io::stdin().read_line(&mut input3).expect("oops, not a valid string");
    let c:f32 = input3.trim().parse().expect("sorry, not a valid number");

    // find the discriminant
    let t:f32 = 2.0;
    let d:f32 = b.powf(t) - (4.0 *(a * c)); 
    // solving for the root now
    if d > 0.0 {
    	let root1 = -b + (d.sqrt()) / (2.0 * a);
    	let root2 = -b - (d.sqrt()) / (2.0 * a);
    	println!("The roots are: {} {}, there are two distinct roots", root1, root2);
    }
    else if d == 0.0 {
    	let root1 = -b + (d.sqrt()) / (2.0 * a);
    	let root2 = -b - (d.sqrt()) / (2.0 * a);
    	println!("The roots are: {} {}, and there is one real root", root1, root2);
    }
    else if d < 0.0 {
    	let root1 = -b + (d.sqrt()) / (2.0 * a);
    	let root2 = -b - (d.sqrt()) / (2.0 * a);
    	println!("The roots are: {} {}, and there are no real roots", root1, root2);
    }
}

