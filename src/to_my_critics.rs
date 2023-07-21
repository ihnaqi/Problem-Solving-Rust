use std::io;

fn input() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();

   input
}

pub fn main(){

   let s = input();
   let mut t = s.trim().parse::<i32>().expect("Couldn't parse");

   while t > 0 {
      solve();
      t = t - 1;
   }

}

fn solve() {

   let mut v: Vec<i32> = input().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();

   for i in 0..v.len() {
      for j in (i+1)..v.len() {
         if v[i] + v[j] >= 10 {
            println!("YES");
            return;
         }
      }
   }
   println!("NO");
}