use std::sync::Mutex;
use std::thread;

fn main() {
	let m = Mutex::new(0); //create a Mutex
	let mut handles = vec![];

	for _ in 0..10 {

		let handle = thread::spawn(move || {
			let mut counter = m.lock().unwrap();
			*counter += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	println!("Result: {}", *m.lock().unwrap());
}

/*
error[E0382]: use of moved value: `m`
  --> ..\30_concurrency.rs:21:25
   |
10 | let handle = thread::spawn(move || {
   |                            ------- value moved (into closure) here
...
21 | println!("Result: {}", *m.lock().unwrap());
   |                         ^ value used here after move
   |
   = note: move occurs because `m` has type `std::sync::Mutex<i32>`, which does not implement the `Copy` trait
*/
