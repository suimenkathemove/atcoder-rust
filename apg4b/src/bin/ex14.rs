use std::cmp::{max, min};

fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32,
    }

    println!("{}", max(max(a, b), c) - min(min(a, b), c));
}
