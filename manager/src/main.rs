use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("manager is running");
        thread::sleep(Duration::from_secs(1));
    }
}
