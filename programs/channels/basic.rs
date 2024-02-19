use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (s, r) = channel();

    thread::spawn(move || {
        for i in 0..100 {
            match s.send(i) {
                Ok(()) => println!("data sent"),
                Err(_) => println!("data sending error"),
            }
        }
    });

    for v in r.into_iter() {
        println!("Got x: {v}")
    }
}
