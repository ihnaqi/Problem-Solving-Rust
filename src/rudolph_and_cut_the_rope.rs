use std::io;

pub fn solution () {

   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("Error occured!");
   let mut t: i32 = s.trim().parse().expect("Error occured while parsing");

   while t > 0 {

      let mut s = String::new();
      io::stdin().read_line(&mut s).expect("Error occured!");
      let mut n: i32 = s.trim().parse().expect("Error occured while parsing");
      s.clear();

      let mut res = 0;

      while n > 0 {
         let mut s = String::new();
         io::stdin().read_line(&mut s).expect("Error occured!");

         let mut numbers = s.trim().split_whitespace();
         let nail_height: i32 = numbers.next().unwrap().parse().expect("Error occured while parsing");
         let rope_height: i32 = numbers.next().unwrap().parse().expect("Error occured while parsing");

         if nail_height - rope_height > 0 {
            res += 1;
         }
         n -= 1;
      }
      println!("{res}");

      t -= 1;
   }

}

#[test]
fn check_the_first_test_case_on_codeforces() {

}
