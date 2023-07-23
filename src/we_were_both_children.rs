//@author: @ihnaqi
//@href: 'https://codeforces.com/contest/1850/problem/F'

use std::io;
use std::cmp;

fn input() -> String {
   let mut s = String::new();
   io::stdin().read_line(&mut s).unwrap();

   s.trim().to_string()
}

pub fn main(){

   let s = input();

   let mut t = s.parse::<i32>().expect("Couldn't be parsed");

   while t > 0 {
      solve();
      t = t - 1;
   }
}

fn solve() {
   let s = input();
   let n = s.parse::<i32>().expect("Couldn't be parsed");

   let mut v: Vec<_> = input().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
   let mut cnt: Vec<i64> = Vec::new();
   let mut mx: Vec<i64> = Vec::new();
   for i in 0..n + 1 {
      cnt.push(0);
      mx.push(0);
   }

   for i in v {
      if i <= n {
         cnt[i as usize] += 1;
      }
   }

   for i in 1..n + 1 {
      let mut j = i;

      while j <= n {
         mx[j as usize] += cnt[i as usize];
         j += i;
      }
   }

   let res = mx.iter().max();
   match res {
      Some(&max) => println!("{}", max),
      None => print!("")
   }
}