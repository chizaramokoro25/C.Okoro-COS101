fn main() {
	let n1 = "Electrical".to_string();
	let n2 = " Electronic".to_string();
	let n3 = " Engineering".to_string();
	let n4 = n1 + &n2 + &n3; // n2 and n3 ref is passed 
	//about Electrical Engineering
    println!("\n The {} is informed by the aspiration to train
    	Electrical/Electronic Engineering professionals in the areas
    	of design, building, and maintenance of electrical concepts", n4);

    let u1 = "Computer".to_string();
    let u2 = " Science".to_string();
    let u3 = u1 + &u2; // u2 refernce is passed
    println!();
    println!("{} is aimedat developing competent, creative, innovative,
    	entrepreneurialand ethically-minded persons, capable of
    	creating valuein the diverse fields of Computer Science", u3);
}
