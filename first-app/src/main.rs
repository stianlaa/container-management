use std::{env, thread};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    loop {
        println!("should be built and pushed first-app running with args: {:?}", args);
        thread::sleep(Duration::from_secs(1));
    }
}
