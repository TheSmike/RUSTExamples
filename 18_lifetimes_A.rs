fn main(){

	//ex1: need lifetime parameter
	{
		let a = 1;
		let b = 2;
		let res = bigger(&a, &b);
		println!("the bigger is {}\n", res);
	}

	//ex2: don't need lifetime parameter
	let s = String::from("apc");
	let first = print_y_and_ref_first(&s, 1);
	println!("first is {}\n", first);

	//ex3: don't need lifetime parameter
	let x = String::from("0123");
	let y = String::from("0123456");
	let longest = longest(x, y);
	println!("longest is {}", longest);
	
}

fn bigger<'a>(a: &'a i64, b: &'a i64) -> &'a i64{ /*, 'b*/
	if *a > *b {
		a
	}else{
		b
	}
} //this compile correctly






fn print_y_and_ref_first(x: &str, y: i64) -> &str {
	println!("y is {}", y);
	x
}


fn longest(x: String, y: String) -> String {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}