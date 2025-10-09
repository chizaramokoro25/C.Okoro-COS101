fn main() {
	let t:f64 = 450_000.00; // qty of toshiba is 2 
	let m:f64 = 1_500_000.00; // qty of mac is 1
	let hp:f64 = 750_000.00; // qty of hp is 3
	let d:f64 = 2_850_000.00; // qty of dell is 3
	let a:f64 = 250_000.00; // qty of acer is 1

	// calculation of the sum
	let sum = t + m + hp + d + a;
	println!("Sum of the sales record is {}", sum);
	// calculation of the average
	let average = sum / 5.0;
	println!("Average of the sales record is {}", average);
	// project 2 completed
}