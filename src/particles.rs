use std::io;
use std::cmp;

pub fn main() {
   let mut s = String::new();

   io::stdin().read_line(&mut s).expect("An error occured!");

   let mut t: i32 = s.trim().parse().expect("Couldn't parse");

   while t > 0 {
      solution();
      t = t - 1;
   }
}

fn solution() {
   let mut s = String::new();

   io::stdin().read_line(&mut s).expect("An error occured!");

   let n: i32 = s.trim().parse().expect("Couldn't parse");

   let mut s = String::new();
   io::stdin().read_line(&mut s).unwrap();
   let mut v: Option<Vec<i64>> = s.split_whitespace().map(|i| i.parse::<i64>().ok()).collect();

   if let Some(v) = v {
      let mut odd_sum = 0;
      let mut even_sum = 0;

      for i in 0..v.len() {
         if i % 2 == 0 {
            odd_sum += cmp::max(v[i], 0);
         }
         else{
            even_sum += cmp::max(v[i], 0);
         }
      }

      let res: i64= cmp::max(odd_sum, even_sum);

      if res == 0 {
         let mut max = v[0];

         for i in 1..v.len() {
            max = cmp::max(max, v[i]);
         }

         println!("{max}");
      }
      else{
         println!("{res}");
      }

   }
   else {
      ()
   }
}