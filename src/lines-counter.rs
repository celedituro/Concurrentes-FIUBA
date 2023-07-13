use std::{thread, fs::File, io::{BufReader, BufRead}};

fn get_filenames() -> Vec<String> {
    let mut filenames = Vec::with_capacity(3);
    for _i in 0..5 {
        filenames.push(r"C:\Users\Asus\Desktop\Facultad\Técnicas de Programación Concurrente\Clases\exercises\src\data\wordcount.txt".to_string());
    }

    filenames
}

fn process_file(filename: String) -> u32 {
    let file = File::open(filename).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let lines = reader.lines();
    
    lines.count() as u32
}

fn main() {
    let filenames = get_filenames();

    let mut handler = Vec::new();
    for filename in filenames {
        handler.push(thread::spawn(move || {
            process_file(filename)
        }));
    }

    let mut result: u32 = 0;
    for handle in handler {
        result += handle.join().unwrap();
    }

    println!("Result: {}", result);
}