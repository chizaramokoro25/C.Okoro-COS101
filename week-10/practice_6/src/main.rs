struct Employee {
	name:String,
	company:String,
	age:u32
}
fn main() {
	let emp1 = Employee {
		company:String::from("Ernst&Young"),
		name:String::from("Ebibiong Jessica"),
		age:25
	};
    print!("Name:    {}\n", emp1.name);
    print!("Company: {}\n", emp1.company);
    print!("Age:     {}", emp1.age);
}
