use std::io;
use std::mem;

static mut A: [i32; 200000] = [0; 200000];

pub fn main(){
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("Error occured");

   let mut t: i32 = s.trim().parse().expect("Couldn't be parsed");

   while t > 0 {
       solution();
       t = t - 1;
   }
}

fn solution(){
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("Error occured");

   let mut n: i32 = s.trim().parse().expect("Couldn't be parsed");

   if n == 1 {
      println!("{}", 1);
   }
   else if n == 2 {
      println!("{} {}", 1, 2);
   }
   else {
      unsafe {
         A.fill(0);
         A[0] = 2;
         A[(n / 2) as usize] = 1;
         A[(n - 1) as usize] = 3;

         let mut c = 4;
         for i in 0..n {
            if A[i as usize] == 0 {
               A[i as usize] = c;
               c += 1;
            }
         }
         for i in 0..n {
            print!("{} ", A[i as usize])
         }
         println!("");
      };
   }
}