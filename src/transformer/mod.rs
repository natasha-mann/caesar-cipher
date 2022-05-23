mod cipher;

pub fn transform_text(input: String, shift: i8)->String {

  let mut result = String::from("");

  for c in input.chars() { 
      let cipher_letter: char = cipher::cipher(c, shift);
      result.push(cipher_letter);
  }

  return result;
}