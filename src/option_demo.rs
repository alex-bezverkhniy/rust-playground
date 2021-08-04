use std::option::Option;

fn safe_division(dividen: u32, division: u32) -> Option<u32> {
  if division == 0 {
    None
  } else {
    Some(dividen / division)
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn test1() {
    assert_eq!(1, super::safe_division(1,1).unwrap());
  }

  #[test]
  fn test_none() {
    assert_eq!(None, super::safe_division(1,0));
    assert_eq!(2, super::safe_division(4,2).unwrap());
  }

  #[test]
  fn test2() {
    assert_eq!(2, super::safe_division(4,2).unwrap());
  }

}