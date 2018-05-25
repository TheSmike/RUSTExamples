#![feature(core_intrinsics)]
mod types;


fn main(){
    //e2: let mut
    let x = 6; 
    println!("The value of x is: {}", x);
    types::print_type(x);
    
    //e2: remove let
    let x = x.to_string();
    println!("The value of x is: {}", x);
    types::print_type(x);

}
