fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0; // r (10%)
	let n:f64 = 5.0; // used n as a sub for t

    // calculation of compound interest
    let a = p * (1.0 + (r /100.0)).powf(n);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("Compound Interest is {}", ci); 
}
	