//@author: @ihnaqi
//@href: https://codeforces.com/contest/1849/problem/B

use std::io;

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

fn solve() {
   let s = input();
   let mut numbers = s.split_whitespace();
   let n = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut k = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");

   let mut vec: Vec<_> = input().split_whitespace().map(|x| (x.parse::<i32>().unwrap()) % k).collect();

   let mut v: Vec<Vec<i32>> = Vec::new();

   for i in 0..n {
      if vec[i as usize] == 0 {
         vec[i as usize] += k;
      }
      v.push(vec![-vec[i as usize], i + 1]);
   }
   vec.clear();
   v.sort();

   for vec in v {
      print!("{} ", vec[1]);
   }

   println!("");

}