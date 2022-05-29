use std::time::{Instant};
use std::fs;
use std::fs::File;
use std::io::Write;

mod transformer;

fn main() {
    const FILENAME:&str = "./prideAndPrejudice.txt";

    // read file and convert contents to &[&str] type
    let data = fs::read_to_string(FILENAME)
    .expect("Something went wrong reading the file");
    let collection = data.split("\n").collect::<Vec<&str>>();  
    let contents: &[&str] = collection.as_slice();

    let shift = 2;

    let start = Instant::now();

    // Run main function
    let result: String = transformer::transform_text(contents, shift);

    let duration = start.elapsed();


    // Create output file and write results
    let mut file = File::create("result.txt").unwrap();
    writeln!(&mut file, "{}", result).unwrap();
    
    // Time taken
    println!("Time elapsed in transform_text function is: {:?}", duration);
}

