fn main(){
	
	let mut v = vec![1, 2, 3, 4, 5];
	{
		let third = &mut v[2]; //get a mutable reference to v
		*third = *third * 3;
	} //third goes out of scope

	for i in &v {  //get an immutable reference to v
		println!("{}", i);
	}

	//compile-time error:  cannot borrow `v` as mutable because it is also borrowed as immutable
}
