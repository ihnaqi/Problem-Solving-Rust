//@author: @ihnaqi
//@href: 'https://codeforces.com/contest/1851/problem/0'

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
   let mut n = numbers.next().unwrap().parse::<i32>().expect("Coldn't Parse");
   let mut m = numbers.next().unwrap().parse::<i32>().expect("Coldn't Parse");
   let mut k = numbers.next().unwrap().parse::<i32>().expect("Coldn't Parse");
   let mut h = numbers.next().unwrap().parse::<i32>().expect("Coldn't Parse");

   let mut v: Vec<_> = input().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

   let mut res = 0;
   for i in v {
      let diff = (h - i).abs();
      for j in 1..m {
         if j * k == diff {
            res += 1;
         }
      }
   }

   println!("{}", res);

}