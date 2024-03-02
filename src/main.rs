use std::fs;
use rand::Rng;
use std::io::prelude::*;

fn main() {

    let mut input_data: Vec<String> = fs::read_to_string("ideas.txt")    // Creates a String
                                    .expect("Failed to read input")
                                    .split("\n")                     // Takes references into the String
                                    .map(|s| s.to_string())
                                    .collect();
    
    input_data.remove(input_data.len() - 1);

    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..input_data.len() as u32);

    println!("The next program you should make is: {}", input_data[n as usize]);

    let mut finished = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("finished.txt")
        .unwrap();

    if let Err(e) = writeln!(finished, "{}", &input_data[n as usize] as &str) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let mut modified_data = input_data.clone();
    modified_data.remove(n as usize);

    let mut ideas = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("ideas.txt")
        .unwrap();

    for idea in modified_data {
        if let Err(e) = writeln!(ideas, "{}", idea) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

}
