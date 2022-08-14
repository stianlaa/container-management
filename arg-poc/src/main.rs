use std::time::Duration;
use std::{env, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("starting arg-poc application");
    loop {
        println!("arg-poc running with args: {:?}", args);
        thread::sleep(Duration::from_secs(1));
    }
}
