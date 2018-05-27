struct Registry<'a> {
	name: &'a str,
	surname : &'a str
}

fn main() {

	let registries = String::from("Billy theKid. Pat Garrett");
	let mut whole = registries.split('.').next().unwrap().split(' ');
	let name = whole.next().unwrap();
	let surname = whole.next().unwrap();

	let me = Registry {
		name: name,
		surname: surname
	};

	println!("registry: {} {}", me.name, me.surname);
}
