fn main() {

	let x = 1;
	let y = -5;
	let ref_to_0;
	
	{
		let z = 0;
		let w = 0;
		let a_ref;
		{
			a_ref = lifetime(&x, &y, &z, &w);
			ref_to_0 = a_ref.0;
		}
		println!("{}", a_ref.1);	
	} // z and w go out of scope

	println!("{}", ref_to_0);
}


fn lifetime<'a, 'b>(x : &'a i64, y : &'a i64, z : &'b i64,  w : &'b i64) -> (&'a i64, &'b i64) {
	if 1 == 1 {
		(x,z)
	} else {
		(y,w)
	}
}