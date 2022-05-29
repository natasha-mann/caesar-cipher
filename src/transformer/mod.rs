
use std::{thread};

mod cipher;

pub fn transform_text(input: &[&str], shift: i8)->String {

  let mut result = Vec::new();

  let chunks =  input.chunks(( input.len() / 50).max(1));
  let mut handles = Vec::new();

  
  for chunk in chunks {

    let string = chunk.join("");

    let handle = thread::spawn(move || {
    let mut chunk_result = String::from("");
      for c in string.chars() { 
        let cipher_letter: char = cipher::cipher(c, shift);
        chunk_result.push(cipher_letter);
    }
   return chunk_result;
    });
    handles.push(handle);
}

 // wait for each thread to finish and combine into the final result
 for handle in handles {
 let unwrapped =  handle.join().unwrap();
  result.push(unwrapped)
}

result.join(" ")
}

