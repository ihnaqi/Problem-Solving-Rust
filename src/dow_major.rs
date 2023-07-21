use std::io;

static mut RES: [char; 1000001] = ['a'; 1000001];

pub fn main() {
   let mut s = String::new();

   io::stdin().read_line(&mut s).expect("An error occured!");

   let mut t = s.trim().parse::<i32>().expect("Couldn't parse");

   while  t > 0 {
       solution();
       t = t - 1;
   }
}

fn solution() {
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("An error occured!");
   let mut n = s.trim().parse::<i32>().expect("Couldn't parse!");

   unsafe{
      let mut c = 1;

      while n % c == 0 {
         c += 1;
      }

      for i in 0..n {
         RES[i as usize] = ('a' as u8 + (i % c) as u8) as char;
         print!("{}", RES[i as usize]);
      }
      println!("");
   }
}