fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function... and so is no longer valid here
    
    let x = 5;                      // x comes into scope
    let y = add_five(x);            // x would move into the function, but i32 is primitive so it would copy instead

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn add_five(some_integer: i32) { // some_integer comes into scope
    let y = some_integer + 5; 
} // Here, some_integer goes out of scope. Nothing special happens because is a copy