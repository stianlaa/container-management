use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("first-app is running");
        thread::sleep(Duration::from_secs(1));
    }
}
