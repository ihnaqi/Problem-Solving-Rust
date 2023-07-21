use std::io;
use std::cmp;

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
   let mut n = s.trim().parse::<i32>().unwrap();
   let mut r: Vec<i32> = Vec::new();
   let mut q: Vec<i32> = Vec::new();

   while n > 0 {
      let s = input();
      let mut numbers = s.trim().split_whitespace();
      let res = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");
      let qual = numbers.next().unwrap().parse::<i32>().expect("Couldn't parse");

      r.push(res);
      q.push(qual);

      n = n - 1;
   }

   let mut max = -1;
   let mut index = 0;
   for i in 0..q.len() {
      if max < q[i] && r[i] <= 10 {
         max = q[i];
         index = i;
      }
   }

   println!("{}", index + 1);

}