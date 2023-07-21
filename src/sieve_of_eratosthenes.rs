use std::io::{self, BufRead};

pub fn sieve() {
    let n: i32 = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let mut table: Vec<bool> = Vec::new();

    for i in 0..(n + 1) {
        table.push(true);
    }

    for i in 2..(n + 1) {
        if table[i as usize] == true {
            let mut j = i * 2;
            for j in ((i * 2)..(n + 1)).step_by(i as usize) {
                table[j as usize] = false;
            }
            print!("{i} ");
        }
    }
    println!();
}
