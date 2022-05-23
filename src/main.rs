use std::time::{Instant};
use std::fs;
use std::fs::File;
use std::io::Write;

mod transformer;

fn main() {
    const FILENAME:&str = "./prideAndPrejudice.txt";

    let contents = fs::read_to_string(FILENAME)
    .expect("Something went wrong reading the file");

    let start = Instant::now();

    let result: String = transformer::transform_text(contents, 2);

    let duration = start.elapsed();

    let mut file = File::create("result.txt").unwrap();

    writeln!(&mut file, "{}", result).unwrap();
  
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

