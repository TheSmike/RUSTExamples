fn main(){             
    
    let s1 = String::from("hello"); 
    println!("{}", s1);
    
    let s2 = s1;  // move 
    println!("{}", s1); //error at compile-time: use of moved value!

} // this scope is now over, and s2 is no longer valid