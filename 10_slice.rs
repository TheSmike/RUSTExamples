fn main() {
	let s = String::from("The Package");
	let the = &s[0..3];
	let package = &s[4..11];
	println!("first '{}', second '{}'", the, package);

	//equals
	let whole1 = &s[0..s.len()];
	let whole2 = &s[..];

	//slices on other types
	let a = [1, 2, 3, 4, 5];
	let slice_a = &a[1..3];
}