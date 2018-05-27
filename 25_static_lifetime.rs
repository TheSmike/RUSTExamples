fn main(){
	let s: &'static str;
	{
		s = "I have a static lifetime.";
	}
	println!("{}", s);
	
}