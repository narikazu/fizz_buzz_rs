fn main() {
  for num in 1..=100 {
    println!("{}",
      if div_by_fifteen(num) { "FizzBuzz".to_string() }
      else if div_by_three(num) { "Fizz".to_string() }
      else if div_by_five(num) { "Buzz".to_string() }
      else { num.to_string() }
    );
  }
}

fn div_by_three(num: i8) -> bool {
  num % 3 == 0
}

fn div_by_five(num: i8) -> bool {
  num % 5 == 0
}

fn div_by_fifteen(num: i8) -> bool {
  num % 15 == 0
}

#[test]
fn test_div_by_three() {
  assert!(!div_by_three(1));
}

#[test]
fn test_div_by_three_with_three() {
  assert!(div_by_three(3));
}

#[test]
fn test_div_by_fifteen() {
  assert!(!div_by_fifteen(1));
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
  assert!(div_by_fifteen(15));
}

#[test]
fn test_div_by_five() {
  assert!(!div_by_five(1));
}

#[test]
fn test_div_by_five_with_five() {
  assert!(div_by_five(5));
}
