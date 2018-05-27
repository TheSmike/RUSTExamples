fn main() {

	let x = 1;
	let y = -5;
	let z = 0;
	let a_ref;
	a_ref = z_sign_select_output(&x, &y, &z);

	/*
	//if condition
	{
		let z = 0;
		a_ref = z_sign_select_output(&x, &y, &z);		
	}
	*/
	

	println!("{}", a_ref);
}


fn z_sign_select_output<'a>(x : &'a i64, y : &'a i64, z : &'a i64) -> &'a i64 {
	if *z > 0 {
		x
	} else {
		y
	}
}