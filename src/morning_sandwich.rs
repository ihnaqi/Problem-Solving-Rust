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
   let b = numbers.next().unwrap().parse::<i32>().expect("Coudln't parse");
   let c = numbers.next().unwrap().parse::<i32>().expect("Coudln't parse");
   let h = numbers.next().unwrap().parse::<i32>().expect("Coudln't parse");

   if h + c >= b {
      println!("{}", b * 2 - 1);
   }
   else {
      println!("{}", (h + c) * 2 + 1 );
   }
}