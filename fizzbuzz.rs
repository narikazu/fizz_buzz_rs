fn div_by_three(num: i8) -> bool {
  num % 3 == 0
}

#[test]
fn test_div_by_three() {
  if div_by_three(1) {
    panic!("One is not three");
  }
}

#[test]
fn test_div_by_three_with_three() {
  if !div_by_three(3) {
    panic!("Three should be three");
  }
}
