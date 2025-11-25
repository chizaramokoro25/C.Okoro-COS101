use::std::fs::File;
use std::io::Write;
fn main() {
	let smis = format (
		"Nigerian Brewery Limited\n
		Portfolio:\n
		Lager\n
		1. 33 Export\n
		2. Desperados\n
		3.Goldberg\n
		4.Guilder\n
		5.Heineken\n
		6.Star
		Stout\n
		1.Legend\n
		2.Turbo KIng\n
		3.Williams\n
		Non-Alcoholic\n
		1.Maltina\n
		2.Amstel Maltina\n
		3.Malta Golda\n
		4.Fayrouz\n"
		);
	let mut file = File::create("High Quality Brewery").expect("invalid input");
	file.write_all(smis.as_bytes()).expect("invalid input");
    println!("Saved all high quality portofolio drinks in txt");
}
