use std::{thread, time::Duration, sync::{Arc, Mutex}};

use rand::Rng;
use std_semaphore::Semaphore;

fn get_product() -> u32 {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..=10);
    
    num
}

fn main() {
    let sem = Arc::new(Semaphore::new(0));
    let clone = sem.clone();
    let products = Arc::new(Mutex::new(Vec::<u32>::new()));
    let products_clone = products.clone();

    // Consumer
    thread::spawn(move || {
        loop {
            println!("[CONSUMER]: waiting");
            sem.acquire();
            {
                let mut prod = products.lock().unwrap();
                let product = prod.remove(0);
                println!("[CONSUMER]: consuming {}", product);
            }
        }
    });

    // Producer
    loop {
        thread::sleep(Duration::from_millis(2000));
        println!("[PRODUCER]: working");
        thread::sleep(Duration::from_millis(3000));
        {
            let mut prod = products_clone.lock().unwrap();
            let product = get_product();
            prod.push(product);
            println!("[PRODUCER]: already produced {}", product);
        }
        clone.release();
    }
}