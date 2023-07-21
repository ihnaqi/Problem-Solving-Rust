use std::io;

pub fn main() {
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("Error Occured");

   let mut t: i32 = s.trim().parse().expect("Couldn't parse");

   while t > 0 {
      solution();
      t -= 1;
   }
}

fn solution () {
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("Error Occured!");

   let mut numbers = s.trim().split_whitespace();

   let a: i32 = numbers.next().unwrap().parse().expect("Couldn't parse");
   let b: i32 = numbers.next().unwrap().parse().expect("Couldn't parse");

   println!("{}", a + b);
}