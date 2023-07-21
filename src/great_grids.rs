use std::io;
use std::cmp;

const fn const_fn() -> Vec<i32> {
   let vt = vec![0, 0];
   let mut v = Vec::from(vt);

   v
}

const VAL: Vec<i32> = const_fn();
static mut V: [Vec<i32>; 4000] = [VAL; 4000];
static mut COLOR: [i32; 4000] = [-1; 4000];

static mut BAD: i32 = 0;

pub fn main () {
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("An error occured");

   let mut t = s.trim().parse::<i32>().expect("Couldn't parse");
   while t > 0 {
       solution();
       t -= 1;
   }
}

fn solution() {
   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("An error occured");
   let mut nmk = s.trim().split_whitespace();

   let mut n = nmk.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut m = nmk.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut k = nmk.next().unwrap().parse::<i32>().expect("Couldn't parse");

   let mut s = String::new();
   io::stdin().read_line(&mut s).expect("An error occured");
   let mut s = s.trim().split_whitespace();

   let mut x1 = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut y1 = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut x2 = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   let mut y2 = s.next().unwrap().parse::<i32>().expect("Couldn't parse");
   x1 -= 1;
   y1 -= 1;
   x2 -= 1;
   y2 -= 1;

   unsafe{
      V[cmp::min(x1, x2) as usize][0] = n + cmp::min(y1, y2);
      V[cmp::min(x1, x2) as usize][1] =
         if x1 + y1 != x2 + y2 {
            0
         }
         else {
            1
         };

      V[(n + cmp::min(y1, y2)) as usize][0] = cmp::min(x1, x2);
      V[(n + cmp::min(y1, y2)) as usize][1]=
         if x1 + y1 != x2 + y2 {
            0
         }
         else {
            1
         };
      COLOR.fill(-1);
   }

   for i in 0..(n + m) {
      if unsafe{COLOR[i as usize] == - 1} {
         dfs(i, 0);
      }
   }

   if unsafe{BAD == 1} {
      println!("NO");
   }
   else {
         println!("YES");
   }

}

fn dfs(u: i32, c: i32) -> i32 {
   if unsafe{
      COLOR[u as usize]
   } != -1 {
      unsafe{
         BAD = 1
      };
      return 0;
   }
   unsafe{
      COLOR[u as usize] = c
   };

   unsafe{

      for v in V.clone() {
         for e in v {
            dfs(u, c ^ e);
         }
      }
      // for (v, mut e) in {
      //    let k = v as usize;
      //    V[k].iter_mut()
      // }.enumerate(){
      //    dfs(u, c^e);
      // }
   }
   0
}