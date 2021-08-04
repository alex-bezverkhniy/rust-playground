use std::option::Option;

fn safe_division(dividen: u32, division: u32) -> Option<u32> {
  Some(0)
}

#[cfg(test)]
mod tests {

  #[test]
  fn test1() {
    assert_eq!(0, super::safe_division(1,1).unwrap());
  }
}