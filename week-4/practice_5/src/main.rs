//Rust Progeram to read the height of a person
// and then print if person is tall, short
// or average height person
use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height (in centimetres):");
    io::stdin().read_line(&mut input).expect("not a valid string");
    let height:f32 = input.trim().parse().expect("not a valid number");

    if height >= 150.0 && height <= 170.0
    {
    	println!("You are an average height person.");
    }
    else if height > 170.0 && height <= 195.0
    {
    	println!("You are tall!");
    }
    else if height < 150.0 && height > 100.0
    {
    	println!("You are a DWARF");
    }
    else 
    {
    	println!("Abnormal height");
    }
}