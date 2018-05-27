use std::fs::File;

fn main(){
	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
	}
	//println!("{}", f.read(1));

}