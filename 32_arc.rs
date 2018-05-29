use std::sync::Arc;
use std::thread;

fn main() {
    let five = Arc::new(String::from("ok"));
    //let five = String::from("five");
    //let ref_five = &five;

    for _ in 0..10 {
        let five = Arc::clone(&five);

        thread::spawn(move || {
            println!("{}", ref_five);
        });
    }
}

