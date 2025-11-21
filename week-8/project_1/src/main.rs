fn public_servant() {
	let staff_table = vec![
	("Office Administrator", "Intern", 1, Some(2), "APS 1-2"),
	("Office Administrator", "Administrator", 3, Some(5), "APS 3-5"),
	("Office Administrator", "Senior Administrator", 5, Some(8), "APS 5-8"),
	("Office Administrator", "Office Manager", 8, Some(10), "ELS 8-10"),
	("Office Administrator", "Director", 10, Some(13), "ELS 10-13"),
	("Office Administrator", "CEO", 13, None, "SES"),
	("Academic", "Research Assistant", 3, Some(5), "APS 3-5"),
	("Academic", "PhD Candidate", 5, Some(8), "APS 5-8"),
	("Academic", "Post-Doc Researcher", 8, Some(10), "ELS 8-10"),
	("Academic", "Senior Lecturer", 10, Some(13), "ELS 10-13"),
	("Academic", "Dean", 13, None, "SES"),
	("Lawyer", "Paralegal", 1, Some(2), "APS 1-2"),
	("Lawyer", "Junior Associate", 3, Some(5), "APS 3-5"),
	("Lawyer", "Associate", 5, Some(8), "APS 5-8"),
	("Lawyer", "Senior Associate 1-2", 8, Some(10), "ELS 8-10"),
	("Lawyer", "Senior Associate 3-4", 10, Some(13), "ELS 10-13"),
	("Lawyer", "Partner", 13, None, "SES"),
	("Teacher", "Placement", 1, Some(2), "APS 1-2"),
	("Teacher", "Classroom Teacher", 3, Some(5), "APS 3-5"),
	("Teacher", "Snr Teacher", 5, Some(8), "APS 5-8"),
	("Teacher", "Leading Teacher", 8, Some(10), "ELS 8-10"),
	("Teacher", "Deputy Principal", 10, Some(13), "ELS 10-13"),
	("Teacher", "Principal", 13, None, "SES")
	];

	println!("Hi, what's your name?");
	let mut name = String::new();
	std::io::stdin().read_line(&mut name).expect("oops, invalid string");
	let name = name.trim().to_lowercase();

	println!("Next, enter your occupation:");
	let mut occ = String::new();
	std::io::stdin().read_line(&mut occ).expect("oops, invalid string");
	let occ = occ.trim().to_lowercase();

	println!("Your job specification?");
	let mut role_input = String::new();
	std::io::stdin().read_line(&mut role_input).expect("oops, invalid string");
	let role_input = role_input.trim().to_lowercase();

	println!("Finally, how many years have you been in the work force?");
	let mut years = String::new();
	std::io::stdin().read_line(&mut years).expect("oops, invalid string");
	let years:u32 = years.trim().parse().expect("soory, invalid input");

	println!("Name: {}", name);
	println!("Occupation: {}", occ);
	println!("Job Specification: {}", role_input);
	println!("Years of Experience: {}", years);

	let mut found = false;
	for (occupation, role, min_years, max_years, level) in &staff_table {
		if occ.eq_ignore_ascii_case(occupation)
		   && role_input.eq_ignore_ascii_case(role)
		   && years >= *min_years
		   && match max_years {
		   	Some(max) => years <= *max,
		   	None => true,
		   }
	    {
	    	println!("Staff level: {}", level);
	    	found = true;
	    	break;
	    }
    }
}
fn main() {
    println!("Hi there, please enter the following information:");
    loop {
    	public_servant();
    	println!("Do you want to enter another record? (yes/no)");
    	let mut again = String::new();
    	std::io::stdin().read_line(&mut again).expect("oops, invalid input");
    	let again = again.trim().to_lowercase();
    	if again != "yes" {
    		break;
    	}
    }
}
