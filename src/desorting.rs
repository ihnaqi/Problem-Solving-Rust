//@author: @ihnaqi
//@href: 'https://codeforces.com/contest/1853/problem/A'

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
      t -= 1;
   }
}

fn solve() {
   let s = input();
   let n = s.parse::<i32>().expect("Couldn't parse");

   let mut v: Vec<_> = input().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();

   let mut sorted_v: Vec<i32> = v.clone();
   sorted_v.sort();

   for i in 0..n {
      if v[i as usize] != sorted_v[i as usize] {
         println!("0");
         return;
      }
   }

   let mut res = 0;

   let mut i = n - 2;
   while i >= 0 {
      let mut l = v[i as usize];
      let mut r = v[(i + 1) as usize];
      let mut diff = r - l;

      let mut div = diff / 2;

      div += 1;

      if res == 0 {
         res = div;
      }
      else {
         if div < res {
            res = div;
         }
      }
      i -= 1;
   }

   println!("{}", res);

}