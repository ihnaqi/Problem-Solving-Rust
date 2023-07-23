//@ihnaqi
//@ref https://codeforces.com/contest/1850/problem/E

use std::io;

fn input() -> String {
   let mut s = String::new();
   io::stdin().read_line(&mut s).unwrap();

   s.trim().to_string()
}

pub fn main() {
   let s = input();
   let mut t = s.parse::<i32>().expect("Coudn't be parsed at the moment");

   while t > 0 {
      solve();
      t = t - 1;
   }
}

fn solve() {
   let s = input();
   let mut numbers = s.split_whitespace();
   let n = numbers.next().unwrap().parse::<i64>().expect("Coundn't be parsed at the moment");
   let c = numbers.next().unwrap().parse::<i64>().expect("Coundn't be parsed at the moment");

   let mut v: Vec<i64> = input().split_whitespace().map(|i| i.parse::<i64>().unwrap()).collect();

   let mut l = 1;
   let mut r = 1e9 as i64;

   while l <= r {
      let mid = l + (r - l) / 2;
      let mut small:i64 = 0;

      for i in 0..n {
         small += (v[i as usize] + 2 * mid) * (v[i as usize] + 2 * mid);

         if small > c {
            break;
         }
      }

      if small == c {
         println!("{}", mid);
         break;
      }

      if small > c {
         r = mid - 1;
      }
      else {
         l = mid + 1;
      }
   }
}