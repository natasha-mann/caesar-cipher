pub fn cipher(letter: char, shift: i8)-> char {

  const ASCII_A: i8 = 'A' as i8;
  const ASCII_Z: i8 = 'Z' as i8;
  const ABC_SIZE: i8 = ASCII_Z - ASCII_A + 1;

      if !letter.is_ascii_alphabetic() {
          return letter;
      }
  
      let uppercase_letter = letter.to_ascii_uppercase();
  
      // --- char to index within alphabet
      let letter_index = uppercase_letter as i8 - ASCII_A;
  
      // --- shift
      let output_index = (letter_index + shift).rem_euclid(ABC_SIZE);
  
      // --- index to char
      let output_upper = ((output_index + ASCII_A) as u8) as char;
  
      if letter.is_ascii_lowercase() {
         return output_upper.to_ascii_lowercase();       
      }

      return output_upper;

}