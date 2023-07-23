//@author: @ihnaqi
//@href: 'https://codeforces.com/contest/1853/problem/B'

use std::io;

fn input() -> String {
   let mut s = String::new();
   io::stdin().read_line(&mut s).unwrap();

   s.trim().to_string()
}

pub fn main() {
   let s = input();
   let mut t = s.parse::<i32>().expect("Couldn't parse");

   while t > 0 {
      solve();
      t = t - 1;
   }
}

fn solve() {

   let s = input();
   let mut numbers = s.split_whitespace();

   let mut n = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut k = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");

   let mut ans = 0;

   for i in 0..(n + 1) {
      if check(n, i, k) == true {
         ans += 1;
      }
   }

   println!("{}", ans);
}

fn check(n: i32, diff: i32, k: i32) -> bool{
   if n < 0 {
      return false;
   }
   if k == 0 {
      return true;
   }

   return check(diff, n - diff, k - 1);
}