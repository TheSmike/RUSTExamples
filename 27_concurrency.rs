use std::thread;
use std::time::Duration;

fn main() {
	//Anonymous function (Closure)
	let handle = thread::spawn(|| {
		//code in async thread
		for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(100));
		}
	});

	//code in main thread
	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(100));
	}

	handle.join().unwrap(); //wait for async thread ends
}
