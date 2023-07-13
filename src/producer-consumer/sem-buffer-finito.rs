use std::{thread, time::Duration, sync::{Arc, Mutex}};

use rand::Rng;
use std_semaphore::Semaphore;

fn get_product() -> u32 {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..=10);
    println!("num produced {}", num);
    num
}

fn init_products(capacity: u32) -> Vec<u32> {
    let mut prod = Vec::new();

    for _i in 0..capacity {
        prod.push(get_product());
    }

    prod
}

const CAPACITY: u32 = 5;

fn main() {
    let empty = Arc::new(Semaphore::new(0));
    let empty_clone = empty.clone();
    let not_full = Arc::new(Semaphore::new(CAPACITY as isize));
    let not_full_clone = not_full.clone();

    let products = Arc::new(Mutex::new(init_products(CAPACITY)));
    let products_clone = products.clone();

    // Consumer
    thread::spawn(move || {
        loop {
            println!("[CONSUMER]: waiting");
            empty.acquire();
            {
                let mut prod = products.lock().unwrap();
                let product = prod.remove(0);
                not_full.release();
                println!("[CONSUMER]: consuming {}", product);
            }
        }
    });

    // Producer
    loop {
        thread::sleep(Duration::from_millis(2000));
        println!("[PRODUCER]: working");
        not_full_clone.acquire();
        thread::sleep(Duration::from_millis(3000));
        {
            let mut prod = products_clone.lock().unwrap();
            let product = get_product();
            prod.push(product);
            println!("[PRODUCER]: already produced {}", product);
        }
        empty_clone.release();
    }
}