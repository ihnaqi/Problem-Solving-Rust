//@author: @ihnaqi
//@href: 'https://codeforces.com/contest/1851/problem/B'

use std::cmp;
use std::io;
use std::mem::swap;

fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim().to_string()
}

pub fn main() {
    let s = input();
    let mut t = s.parse::<i32>().expect("Couldn't parse");

    while t > 0 {
        solve();
        t = t - 1;
    }
}

fn solve() {
    let s = input();
    let mut n = s.parse::<i32>().expect("Couldn't parse");

    let mut v: Vec<_> = input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut v_res = Vec::new();

    {
        let mut v_odd: Vec<i32> = Vec::new();
        let mut v_even: Vec<i32> = Vec::new();

        for i in v.clone() {
            if i % 2 == 0 {
                v_even.push(i);
            } else {
                v_odd.push(i);
            }
        }

        v_even.sort();
        v_odd.sort();

        let mut odd_counter = 0;
        let mut even_counter = 0;

        for i in 0..n {
            if v[i as usize] % 2 == 0 {
                v_res.push(v_even[even_counter]);
                even_counter += 1;
            } else {
                v_res.push(v_odd[odd_counter]);
                odd_counter += 1;
            }
        }

        v.clear();
        v_even.clear();
        v_odd.clear();
    }

    if v_res.windows(2).all(|window| window[0] <= window[1]) {
        println!("YES");
    } else {
        println!("NO");
    }
}
