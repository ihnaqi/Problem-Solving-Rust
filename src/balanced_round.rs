use std::io;
use std::cmp;

fn input() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();

   input
}

pub fn main() {
   let s = input();
   let mut t = s.trim().parse::<i32>().expect("Couldn't parse");

   while t > 0 {
      solve();
      t = t - 1;
   }
}

fn solve() {
   let s = input();
   let mut numbers = s.trim().split_whitespace();
   let n = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let k = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");

   let mut v: Vec<i32> = input().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
   v.sort();

   let mut count = 1;
   let mut res = 0;

   for i in 1..n {
      if((v[(i - 1) as usize] - v[i as usize]).abs() <= k) {
         count += 1;
      }
      else {
         res = cmp::max(res, count);
         count = 1;
      }
   }
   res = cmp::max(res, count);

   println!("{}", (n - res));
}
