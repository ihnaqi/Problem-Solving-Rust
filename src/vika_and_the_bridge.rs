use std::io;
use std::cmp;

fn input() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();

   input
}

pub fn main() {
   let s = input();
   let mut t = s.trim().parse::<i32>().unwrap();

   while t > 0 {
      solve();
      t -= 1;
   }
}

fn solve() {
   let s = input();
   let mut s = s.split_whitespace();
   let n = s.next().unwrap().parse::<i32>().unwrap();
   let k = s.next().unwrap().parse::<i32>().unwrap();

   let mut c: Vec<i32> = input().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();

   let mut last = Vec::with_capacity(k as usize);
   let mut max_step: Vec<i32> = Vec::with_capacity(k as usize);
   let mut max_step2: Vec<i32> = Vec::with_capacity(k as usize);
   for i in 0..k {
      last.push(-1);
      max_step.push(0);
      max_step2.push(0);
   }


   for i in 0..n {
      let mut step = i - last[(c[i as usize] - 1) as usize];
      if step > max_step[(c[i as usize] - 1 ) as usize] {
         max_step2[(c[i as usize] - 1) as usize] = max_step[(c[i as usize] - 1) as usize];
         max_step[(c[i as usize] - 1) as usize] = step;
      }
      else if step > max_step2[(c[i as usize] - 1) as usize] {
         max_step2[(c[i as usize] - 1) as usize] = step;
      }
      last[(c[i as usize] - 1) as usize] = i;
   }

   for i in 0..k {
      let mut step = n - last[i as usize];

      if step > max_step[i as usize] {
         max_step2[i as usize] = max_step[i as usize];
         max_step[i as usize] = step;
      }
      else if step > max_step2[i as usize] {
         max_step2[i as usize] = step;
      }
   }

   let mut ans = 1e9 as i32;

   for i in 0..k {
      ans = cmp::min(ans, cmp::max((max_step[i as usize] + 1) / 2, max_step2[i as usize]));
   }

   println!("{}", ans - 1);
}
