fn main(){
	let x = 1;
    println!("x = {}", x);
    
	x = + 1; // ERROR at compile-time
    println!("x = {}", x);
} 

const MONTH_NUMBER: u8 = 12;