use std::thread;
use std::time::Duration;

fn main() {
    let first_val = 3;
    let second_val = 2;
    println!(
        "calculating: {first_val}*{second_val}={}",
        first_val * second_val
    );
    thread::sleep(Duration::from_secs(5));
    panic!("This container is to see docker behaviour of a container panicking and unwinding")
}
