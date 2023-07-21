fn nth(n: u32) -> u32 {
  
  let mut total_primes = 0;
  let mut size_factor = 2;

  let mut s = n * size_factor;
  let mut primes: Vec<u8> = Vec::new();
  while total_primes < n {
    primes = get_primes(s);

    total_primes = primes[2..].iter().sum();
    size_factor += 1;

    s = n * size_factor;
  }

  let mut count: u32 = 0;
  for k in 0..primes.len() {
    count += primes[k] as u32;
    
    if count == n {
      count = k as u32;
      break;
    }
  }
  count
}

fn get_primes(s: u32) -> Vec<u8> {
  let mut primes: Vec<u8> = Vec::new();
  
  for i in 0..s {
    primes.push(1);
  }

  for i in 2..s {
    if primes[i as usize] == 1 {
      for j in i..s {
        if i * j < s {
          primes[(i * j) as usize] = 0;
        }
        else {
          break;
        }
      }
    }
   }

  primes
}



#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}
#[test]
// #[ignore]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}
#[test]
// #[ignore]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}
#[test]
// #[ignore]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}


