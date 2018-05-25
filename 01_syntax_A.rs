fn main() {
	//definizione implicita
	let decimal_impl = 16; 
	//definizione esplicita 
	let decimal_espl : i32 = 16;
	//notazione esadecimale
	let decimal_hex_form = 0x10;
	//ci sono ovviamente molti altri tipi e notazioni...
	println!("\ndecimal_impl {} \ndecimal_espl {} \ndecimal_hex_form {}\n", decimal_impl, decimal_espl, decimal_hex_form);

	//floating point 
	let double = 3.0;
	let single : f32 = 1.0;

	//tupla
	let tup_expl : (i32, f64, u8, char) = (500, 6.4, 1, 'a');
	let tup_impl = (500, true);
	println!("Tuple:{},{},{},{}", tup_expl.0, tup_expl.1, tup_expl.2, tup_expl.3);

	//array 
	let a = [1, 2, 3, 4, 5];
	println!("array's first value: {}\n", a[0]);


	let text = String::from("I'm a text!");
	println!("text: {}\n", text);

	let five = abs(-5);
	println!("abs of -5: {}\n", five);

	println!("abs(-5) + 3 =  {}\n", add_three(five));
	
	try_loops();

}

fn add_three(x:i32)-> i32{
	x+3
}

fn abs(x : i32) -> i32 {
	if x >= 0{
		return x;
	}
	else {
		return x * (-1);
	}
}

fn assign_when_true(condition: bool, old_value: i64) -> i64 {
	let number = if condition {
		100
	} else {
		old_value
	};

	number
}

fn try_loops(){
	loop {
        println!("again!");
    }

	let mut number = 5;
	while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
	println!("END!");

	let a = [10, 20, 30, 40, 50];
	for element in a.iter() {
        println!("the value is: {}", element);
    }

	for number in (1..4).rev() {
        println!("countdown {}!", number);
    }

}
