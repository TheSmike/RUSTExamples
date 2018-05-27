fn main() {
	
	let condition = true; //from input, can change
	let string1 = String::from("Contrary to popular belief, Lorem Ipsum is not simply random text.");

	//compute
    if condition {
        let short = String::from("short!");
        let result = longest(string1.as_str(), short.as_str());
		println!("The longest string is '{}'", result);
    } else {
		let medium = String::from("medium long text");
		let result; 
        result = longest(string1.as_str(), medium.as_str());
		println!("The longest string is '{}'", result);
	}
    //move result and println out...
}



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}