use std::collections::HashMap;

fn pangrams(s: &str) -> String {
  let mut cache = [0; 26];
  let mut sum = 0;

  for c in s.chars() {
    if c.is_ascii_alphabetic() {
      match (c.to_ascii_lowercase() as u8).checked_sub(b'a') {
        Some(value) => {
          cache[value as usize] = 1;
        },
        None => println!("Subtraction resulted in overflow/underflow"),
      }
    }
  }
  for i in cache {
    sum += i;
  }

  if sum == 26 {String::from("pangram")} else {String::from("not pangram")}
}

#[test]
fn sample_test_case_1() {
  let s = "We promptly judged antique ivory buckles for the next prize";

  let original_res = String::from("pangram");

  assert_eq!(original_res, pangrams(&s));
}

#[test]
fn sample_test_case_2() {
  let s = "A quick brown fox jumps over the lazy dog";

  let original_res = String::from("pangram");

  assert_eq!(original_res, pangrams(&s));
}

#[test]
fn sample_test_case_3() {
  let s = "We promptly judged antique ivory buckles for the prize";

  let original_res = String::from("not pangram");

  assert_eq!(original_res, pangrams(&s));
}