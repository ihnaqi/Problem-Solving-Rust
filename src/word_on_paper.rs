use std::io;

fn input() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();

   input
}

pub fn main() {
   let s = input();
   let mut t = s.trim().parse::<i32>().expect("Couldn't parse");

   while t > 0 {
      solve();
      t = t - 1;
   }
}

fn solve() {
   let mut res = String::new();
   for i in 0..8 {
      let s = input();
      let c = find_letter(s);
      match c{
         Some(value) => res.push(value),
         None => print!("")
      }
   }

   println!("{}", res);

}

fn find_letter(s: String) -> Option<char> {
   let mut alphabet = None;

   for c in s.chars() {
      if c.is_alphabetic() {
         alphabet = Some(c);
         break;
      }
   }
   alphabet
}