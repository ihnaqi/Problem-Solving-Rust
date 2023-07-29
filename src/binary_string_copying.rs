//@author: @ihnaqi

use std::hash::Hasher;
use std::{io, mem::swap, cmp, cmp::Ordering};
use std::collections::HashMap;


fn input() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();

   input
}

pub fn main(){
   let s = input();
   let mut t = s.trim().parse::<i32>().unwrap();

   while t > 0 {
      solve();
      t -= 1;
   }
}

#[derive(Hash, PartialOrd)]
struct Pair {
   first: usize,
   second: usize
}

impl Pair {
   fn new(first: usize, second: usize) -> Self {
      Self {first, second}
   }
}

impl Eq for Pair {}


impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl Ord for Pair {
   fn cmp(&self, other: &Self) -> Ordering {
      self.first.cmp(&other.first)
   }
}

fn solve() {
   let s = input();
   let mut numbers = s.split_whitespace();
   let n = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut m = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");

   let mut s: Vec<char> = input().chars().collect();
   let mut prefix: Vec<usize> = Vec::new();
   let mut suffix: Vec<usize> = Vec::new();

   let mut check = false;
   let mut prev: usize = 1 << 29;

   for i in (n..=0).rev() {
      if s[i as usize] == '0' {
         prev = i as usize;
      }
      suffix[i as usize] = prev;
   }

   prev = 0;

   for i in 0..n {
      if s[i as usize] == '0' {
         prev = i as usize;
      }
      prefix[i as usize] = prev;
   }

   let mut ans: usize = 0;

   let mut map: HashMap<Pair, bool> = HashMap::new();

   for i in 0..n {
      let s = input();
      let mut numbers = s.split_whitespace();
      let mut l = numbers.next().unwrap().parse::<usize>().unwrap();
      let mut  r = numbers.next().unwrap().parse::<usize>().unwrap();

      l = cmp::max(l, suffix[l]);
      r = cmp::min(r, prefix[r]);

      if r < l {
         if !check {
            ans += 1;
            check = true;
         }
         continue;
      }

      let p = Pair::new(l, r);
      match map.get(&p) {
         Some(&mut value) => {
            ans += 1;
            value.replace(true);
         },
         None => {
            map.insert(Pair::new(l, r), true);
         }
      }
   }
}