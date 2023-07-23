//@author: @ihnaqi
//@href: 'https://codeforces.com/contest/1853/problem/C'

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

   let mut v: Vec<_> = input().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();

   let mut curr = 0;
   let mut curv = 1;

   for i in 0..k {
      curv += curr;
      while curr < n && curv >= v[curr as usize] {
         curr += 1;
         curv += 1;
      }
   }

   println!("{}", curv);

}
