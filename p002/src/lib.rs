use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;
// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.

// https://doc.rust-lang.org/std/sync/struct.Mutex.html

pub fn thread_v1 () {
  let data = Arc::new(Mutex::new(0));
  
  let (tx, rx) = channel();

  for _ in 0..N {
    let (data, tx) = (Arc::clone(&data), tx.clone());

    thread::spawn(move || {
      let mut data = data.lock().unwrap();

      *data += 1;

      if *data == N {
        tx.send(()).unwrap();
      }

    });
  }
  println!("data: {:?}", data);
  rx.recv().unwrap();
}