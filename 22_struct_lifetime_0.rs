struct Registry {
	name: & str,
	surname : & str
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

/*
error[E0106]: missing lifetime specifier
 --> ..\22_struct_lifetime.rs:2:8
  |
2 |     name: & str,
  |           ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 --> ..\22_struct_lifetime.rs:3:12
  |
3 |     surname : & str
  |               ^ expected lifetime parameter
*/