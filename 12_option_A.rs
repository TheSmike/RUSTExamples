fn main(){
	
	//ex1
	let some_number = Some(3);
	let some_string = Some("text");
	let absent_number: Option<i32> = None;

	//ex2
	let x: i8 = 7;
	let y: Option<i8> = Some(3);

	//let sum = x + y;
	let sum = x + y.unwrap_or(0);

	//ex3
	let result1 = divide(5.0, 7.0);
	let result2 = divide(5.0, 0.0);
	println!("first is some? = {}, second is some? = {}", result1.is_some(), result2.is_some() );
	
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
