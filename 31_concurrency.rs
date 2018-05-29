use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
let m = Arc::new(Mutex::new(0)); // create an Arc object, object with multiple owner 
let mut handles = vec![];

for _ in 0..10 {
let m_ref = Arc::clone(&m); // increment counter of owner to mutex and return the reference to mutex
let handle = thread::spawn(move || {
let mut counter = m_ref.lock().unwrap();
*counter += 1;
});
handles.push(handle);
}

for handle in handles {
handle.join().unwrap();
}

println!("Result: {}", *m.lock().unwrap());
}