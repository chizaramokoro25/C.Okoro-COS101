use::std::fs::File;
use std::io::{self, Write};

fn pau_smis(prompt: &str) -> String {
	println!("{}", prompt);
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).expect("oops, wrong input");
	let value = input.trim().to_string();
	value
}
fn main() {
    let mut smis: Vec<(String, String, String, u32, u32, f32, f32, f32, f32)> = Vec::new();
    loop {
    	println!("Add a Student:");

    	let name = pau_smis("Name:");
    	let matric = pau_smis("Matric Number:");
    	let department = pau_smis("Department:");
    	let year:u32 = pau_smis("Birth Year:").parse().unwrap();
    	let level:u32 = pau_smis("Level(e.g,100,200...):").parse().unwrap();
    	let input1:f32 = pau_smis("CA 1").parse().unwrap();
    	let input2:f32 = pau_smis("CA 2").parse().unwrap();
    	let input3:f32 = pau_smis("Exam Score").parse().unwrap();
    	let average = (input1 + input2 + input3) / 3.0;
    	println!("Average Score for {}: {:.2}", name, average);

    	smis.push((
    		name,
    		matric,
    		department,
    		year,
    		level,
    		input1,
    		input2,
    		input3,
    		average
    	));
    	let again = pau_smis("Add another? (yes/no)");
    	if again != "yes" {
    		break;
    	}
    }
    let mut file = File::create("Students_managment_system.csv").unwrap();
    file.write_all("name, matric number, department, birth year, level, ca1, ca2, ca3, average".as_bytes()).unwrap();

    for s in &smis {
    	let row = format!("\n{}, {}, {}, {}, {}, {:.2}, {:.2}, {:.2}, {:.2}\n", s.0, s.1, s.2, s.3, s.4, s.5, s.6, s.7, s.8);
    	file.write_all(row.as_bytes()).unwrap();
    }
    println!("\nSaved all students information"); 
}
