use std::io;
struct Item {
	name:&'static str,
	code:&'static str,
	price:f64,
}
fn main() {
	loop {
		let item = [
		Item{name: "Pounded Yam/Edikaikong Soup", code:"p", price: 3_200.00},
		Item{name: "Fried rice and Chicken", code: "F", price: 3_000.00},
		Item{name: "Amala & Ewedu Soup", code: "A", price: 2_500.00},
		Item{name: "Eba & Egusi Soup", code: "E", price: 2_000.00},
		Item{name: "White rice & steew", code: "w", price: 2_500.00},
		];
		println!("Available Items:");
		for item in &item {
			println!("Name: {} / Code: {} / Price: {}", item.name, item.code, item.price);
        }
		println!("Enter item code:");
		let mut code = String::new();
		io::stdin().read_line(&mut code).expect("invalid string");
		let code = code.trim().to_lowercase();

		let mut found = true;
        for item in &item {
        	if item.code == code {
        		found = true;
        		
        		println!("Quantity");
        		let mut qty = String::new();
        		io::stdin().read_line(&mut qty).expect("oops, invalid string");
        		let qty:u32 = qty.trim().parse().expect("invalid input");

        		let total = item.price * qty as f64;
        		let final_total = if total > 10_000.00{
        			total * 0.95
        		} else {
        			total
        		};

        		println!("Item: {} \nQuantity: {} \nSubTotal: {} \nTotal: {}", item.name, qty, total, final_total); 
        	}
        }
        if !found {
        	println!("Item Code not found");
        }
        println!("Do you want to enter another record? (yes/no)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("invalid input");
        let again = again.trim().to_lowercase();
        if again != "yes" {
        	break;
        }
	}
}
