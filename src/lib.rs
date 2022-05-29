mod transformer;

#[cfg(test)]
mod tests {
    use crate::transformer;

    #[test]
    // test for all lowercase input
    fn lowercase_transform() {
      let string = "abcde".split("\n").collect::<Vec<&str>>();
      let str = string.as_slice();
      let result: String = transformer::transform_text(str, 2);
      assert_eq!(result, "cdefg");
    }

    #[test]
    // test for all uppercase input
    fn uppercase_transform() {
      let string = "ABCDE".split("\n").collect::<Vec<&str>>();
      let str = string.as_slice();
      let result: String = transformer::transform_text(str, 2);
      assert_eq!(result, "CDEFG");
    }

    #[test]
    // test for mixed case input
    fn mixed_transform() {
      let string = "ZyxWvABc".split("\n").collect::<Vec<&str>>();
      let str = string.as_slice();
      let result: String = transformer::transform_text(str, 2);
      assert_eq!(result, "BazYxCDe");
    }

    #[test]
    // test for mixed case input with punctuation
    fn punctuation_transform() {
      let string = "Z!yxW.vABc".split("\n").collect::<Vec<&str>>();
      let str = string.as_slice();
      let result: String = transformer::transform_text(str, 2);
      assert_eq!(result, "B!azY.xCDe");
    }

 
}