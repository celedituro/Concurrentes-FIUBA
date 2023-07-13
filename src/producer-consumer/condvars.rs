use std::{thread, time::Duration, sync::{Arc, Condvar, Mutex}};

use rand::Rng;

fn get_product() -> u32 {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..=10);

    num
}

fn main() {    
    let condvar = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let clone = condvar.clone();
    
    // Consumer
    thread::spawn(move || {
        loop {
            println!("[CONSUMER]: waiting");
            let mut guard = condvar.1.wait_while(condvar.0.lock().unwrap(), |pending| {
                println!("[CONSUMER]: checking condition {:?}", *pending);
                pending.is_empty()
            }).unwrap();
            println!("[CONSUMER]: consuming");
            guard.remove(0);
        }
    });

    // Producer
    loop {
        thread::sleep(Duration::from_millis(2000));
        println!("[PRODUCER]: working");
        thread::sleep(Duration::from_millis(3000));
        {
            let mut prod = clone.0.lock().unwrap();
            let product = get_product();
            prod.push(product);
            println!("[PRODUCER]: already produced {}", product);
            clone.1.notify_all();
        }
    }
}