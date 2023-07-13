use std::{thread, time::Duration, sync::{Arc, Barrier}};

const WORKERS: u32 = 5;
fn main() {
    let barrier = Arc::new(Barrier::new(WORKERS as usize));
    
    let mut handler = Vec::new();
    for id in 0..WORKERS {
        let clone = barrier.clone();
        handler.push(thread::spawn(move || {
            println!("[WORKER {:?}]: waiting", id);
            clone.wait();
            thread::sleep(Duration::from_secs(5));
            println!("[WORKER {:?}]: finish", id);
        }));
    }

    for handle in handler {
        handle.join().unwrap();
    }
}