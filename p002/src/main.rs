mod lib;
use lib::thread_v1;
use lib::mutex_v2;

fn main() {
    thread_v1();
    mutex_v2();
    println!("Hello, world!");
}
