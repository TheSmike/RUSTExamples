fn main() {
    let t1  = String::from("First text");    
    let t2  = String::from("Second text");    

    let mut r1 = &t1;
    borrow_string(r1);
    r1 = &t2;
    borrow_string(r1);


}

fn borrow_string(s: &String){
    println!("borrow string is '{}'",s);
}

