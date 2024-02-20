use std::str;

fn main() {
    let name = "Ahllow afafa f....";

    let ptr = name.as_ptr();

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, 50);
        let d = str::from_utf8(slice);
        println!("{:?}", d);
    }

}