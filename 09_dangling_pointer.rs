fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

/*
PS D:\Universita\PLP\RUSTExamples\target> rustc ..\09_dangling_pointer.rs
error[E0106]: missing lifetime specifier
 --> ..\09_dangling_pointer.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
  = help: consider giving it a 'static lifetime

error: aborting due to previous error
*/