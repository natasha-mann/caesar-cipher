mod transformer;

#[cfg(test)]
mod tests {
    use crate::transformer;

    #[test]
    fn lowercase_transform() {
      let string = String::from("abcde");
      let result: String = transformer::transform_text(string, 2);
      assert_eq!(result, "cdefg");
    }

    #[test]
    fn uppercase_transform() {
      let string = String::from("ABCDE");
      let result: String = transformer::transform_text(string, 2);
      assert_eq!(result, "CDEFG");
    }

    #[test]
    fn mixed_transform() {
      let string = String::from("ZyxWvABc");
      let result: String = transformer::transform_text(string, 2);
      assert_eq!(result, "BazYxCDe");
    }

    #[test]
    fn punctuation_transform() {
      let string = String::from("Z!yxW.vABc");
      let result: String = transformer::transform_text(string, 2);
      assert_eq!(result, "B!azY.xCDe");
    }

 
}