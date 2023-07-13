use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rand::Rng;

fn num_aleatorio() -> u32 {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..=10);
     num
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // Consumer
    thread::spawn(move | | {
        loop {
            println!("[CONSUMER]: waiting");
            let product= rx.recv().unwrap();
            println!("[CONSUMER]: product received {:?}", product);
        }
    });	
    
    // Producer
    loop {
        thread::sleep(Duration::from_millis(5000));
        println!("[PRODUCER]: send product");
        tx.send(num_aleatorio()).unwrap();
    };
}