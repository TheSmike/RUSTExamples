fn main(){
	//reference to elements of a vector
	let mut v = vec![1, 2, 3, 4, 5];
	let first = &v[0];
	v.push(6); 	//compile-time error:  cannot borrow `v` as mutable because it is also borrowed as immutable
}
