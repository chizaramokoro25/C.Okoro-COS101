fn main() {
	let t1:f64 = 450_000.00;
	let t2:f64 = 450_000.00; // qty of toshiba is 2 
	let m:f64 = 1_500_000.00; // qty of mac is 1
	let hp1:f64 = 750_000.00;
	let hp2:f64 = 750_000.00;
	let hp3:f64 = 750_000.00; // qty of hp is 3
	let d1:f64 = 2_850_000.00;
	let d2:f64 = 2_850_000.00;
	let d3:f64 = 2_850_000.00; // qty of dell is 3
	let a:f64 = 250_000.00; // qty of acer is 1

	// calculation of the sum
	let sum = t1 + t2 + m + hp1 + hp2 + hp3 + d1 + d2 + d3 + a;
	println!("Sum of the sales record is {}", sum);
	// calculation of the average
	let average = sum / 10.0;
	println!("Average of the sales record is {}", average);
}