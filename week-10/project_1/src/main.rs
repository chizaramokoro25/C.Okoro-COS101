struct Laptop{
	hp:f32,
	ibm:f32,
	toshiba:f32,
	dell:f32
}
impl Laptop {
	fn you_hp(&self)-> f32 {
		self.hp * 3.0
	}
	fn you_ibm(&self)-> f32 {
		self.ibm * 3.0
	}
	fn you_toshiba(&self)-> f32 {
		self.toshiba * 3.0
	}
	fn you_dell(&self)-> f32 {
		self.dell * 3.0 
	}
	fn cost(&self)-> f32 {
		(self.hp * 3.0) + (self.ibm * 3.0) + (self.toshiba *3.0) + (self.dell * 3.0)
	}
}

fn main() {
	let total = Laptop{
		hp:650_000.00,
		ibm:755_000.00,
		toshiba:550_000.00,
		dell:850_000.00
	};
    println!("****************************");
    println!("Hp:     650,000:3--- {}", total.you_hp());
    println!("IBM:    755,000:3--- {}", total.you_ibm());
    println!("Toshiba:550,000:3--- {}", total.you_toshiba());
    println!("Dell:   850,000:3--- {}", total.you_dell());
    println!("Total:        :12--- {}", total.cost());
    println!("------------------------------");
    println!("Thank you for shopping with us!");
}