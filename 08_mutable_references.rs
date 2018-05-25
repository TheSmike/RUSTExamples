fn  main() {
    let mut s = String::from("mutable string! ");


    let r1 = &mut s;
    borrow_and_edit_string(r1);
    println!("{}", r1);

    //e2
    //let r2 = &s;
    //let r3 = &s;
    //let r4 = &mut s;

}

fn borrow_and_edit_string(s: &mut String){
    s.push_str("_ADDED_TEXT");
}

