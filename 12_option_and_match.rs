fn main(){

	//ex1
	let input_value = 5;
	let mut value : Option<i64> = Some(input_value);
	value = match value {
        None => None,
        Some(i) => Some(i + 1),
    };
	println!("{}", value.unwrap_or(0));
	
}
