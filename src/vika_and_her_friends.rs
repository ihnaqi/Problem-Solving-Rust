use std::io;

pub fn main() {
   let mut s = input();

   let mut t = s.trim().parse::<i32>().expect("Couldn't parse");

   while t > 0 {
      solution();

      t -= 1;
   }
}

fn solution() {
   let s = input();
   let mut s = s.trim().split_whitespace();

   let n = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let m = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut k = s.next().unwrap().parse::<usize>().expect("Couldn't parse");

   let s = input();

   let mut s = s.trim().split_whitespace();
   let x = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let y = s.next().unwrap().parse::<i32>().expect("Couldn't parse");

   let mut res = "YES";

   while k > 0 {
      let s = input();
      let mut s = s.trim().split_whitespace();
      let xi = s.next().unwrap().parse::<i32>().unwrap();
      let yi =  s.next().unwrap().parse::<i32>().unwrap();
      if (x + y) % 2 == (xi + yi) % 2 {
         res = "NO";
      }
      k -= 1;
   }

   println!("{}", res);

}

fn input() -> String {
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();

   return input;
}