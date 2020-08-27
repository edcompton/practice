fn fizzbuzz(i: u32) -> String {
  if i % 3 == 0 && i % 5 == 0 {
    String::from("FizzBuzz")
  } else if i % 5 == 0 {
    String::from("Buzz")
  } else if i & 3 == 0 {
    String::from("Fizz")
  } else {
    format!("{}", i)
  }
}

fn main() {
  for i in 1..=100 {
    println!("{}", fizzbuzz(i));
  }
}
