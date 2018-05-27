fn main(){

	//create vector with macro constructor
	let v = vec![1, 2, 3, 4, 5];

	//create vector and push elements (mutable var)
	let mut v = Vec::new();
	v.push(5);
	v.push(10);
	v.push(15);

	//access values
	let value100 : Option<&i32> = v.get(100);
	println!("value100 is none? {}", value100.is_none() );
	
	let value100 : &i32 = &v[100];
	println!("{}", value100 );

}
