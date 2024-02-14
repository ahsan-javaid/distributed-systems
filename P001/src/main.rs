use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {

    let name = String::from("hello john");

    let m = Mutex::new(name);

    let shared_string = Arc::new(m);

    let t1_data = shared_string.clone();
    let t2_data = shared_string.clone();

    let t1 = thread::spawn(move || {
        let mut data = t1_data.lock().unwrap();
        data.push_str(" From Thread 1");
    });

    let t2= thread::spawn(move || {
        let mut data = t2_data.lock().unwrap();
        data.push_str(" from thread 2");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let data = shared_string.lock().unwrap();

    println!("Final data is here: {}", *data);
}
