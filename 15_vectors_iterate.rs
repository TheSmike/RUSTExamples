fn main(){
	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}

	//let v = v; //I can remove mutability
	for i in &v {
		println!("{}", i);
	}
	
}
