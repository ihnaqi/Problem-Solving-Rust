use std::io;

pub fn main() {
   let mut t = String::new();
   io::stdin().read_line(&mut t).expect("Error occured!");

   let mut t: i32 = t.trim().parse().expect("Error occured while Parsing");

   while (t > 0) {
      solution();
      t -= 1;
   }
}

fn solution(){
   let mut a = String::new();
   let mut b = String::new();
   let mut c = String::new();

   io::stdin().read_line(&mut a).expect("Error occured!");
   io::stdin().read_line(&mut b).expect("Error occured!");
   io::stdin().read_line(&mut c).expect("Error occured!");

   let mut res = Some('.');

   let r1 = win_condition(&a.chars().nth(0), &a.chars().nth(1), &a.chars().nth(2));
   if r1 {
      res = a.chars().nth(0);
   }

   let c1 = win_condition(&a.chars().nth(0), &b.chars().nth(0), &c.chars().nth(0));
   if c1 {
      res = a.chars().nth(0);
   }

   let ld = win_condition(&a.chars().nth(0), &b.chars().nth(1), &c.chars().nth(2));
   if ld {
      res = a.chars().nth(0);
   }

   let c2 = win_condition(&a.chars().nth(1), &b.chars().nth(1), &c.chars().nth(1));
   if c2 {
      res = a.chars().nth(1);
   }

   let r2 = win_condition(&b.chars().nth(0), &b.chars().nth(1), &b.chars().nth(2));
   if r2 {
      res = b.chars().nth(0);
   }

   let c3 = win_condition(&a.chars().nth(2), &b.chars().nth(2), &c.chars().nth(2));
   if c3 {
      res = a.chars().nth(2);
   }

   let r3 = win_condition(&c.chars().nth(0), &c.chars().nth(1), &c.chars().nth(2));
   if r3 {
      res = c.chars().nth(0);
   }

   let rd = win_condition(&a.chars().nth(2), &b.chars().nth(1), &c.chars().nth(0));
   if rd {
      res = c.chars().nth(0);
   }


   match res {
      Some(val) => {
         if val == '.' {
            println!("DRAW");
         }
         else {
            println!("{val}")
         }
      },
      None => ()
   }

}

fn win_condition(a: &Option<char>, b: &Option<char>, c: &Option<char>) -> bool {
   a == b && a == c && a != &Some('.')
}
