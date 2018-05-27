struct Registry<'a> {
	name: &'a str,
	surname : &'a str
}

fn main() {

	let me = get_first();

	println!("registry: {} {}", me.name, me.surname);
}

fn get_first<'a>() -> Registry<'a> { //added notation
	let registries = String::from("Billy theKid. Pat Garrett");
    let mut whole = registries.split('.').next().unwrap().split(' ');
	let name = whole.next().unwrap();
	let surname = whole.next().unwrap();

	let me = Registry {
		name: name,
		surname: surname
	};
	me
}