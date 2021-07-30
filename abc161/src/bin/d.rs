use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        K: usize,
    }

    let mut count = 0;

    let mut q: VecDeque<usize> = VecDeque::new();

    for i in 1..=9 {
        q.push_back(i);
    }

    while let Some(i) = q.pop_front() {
        count += 1;

        if count >= K {
            println!("{}", i);
            return;
        }

        let i_first_place = i % 10;

        if i_first_place > 0 {
            let new_i = format!("{}{}", i, i_first_place - 1).parse().unwrap();
            q.push_back(new_i);
        }

        let new_i = format!("{}{}", i, i_first_place).parse().unwrap();
        q.push_back(new_i);

        if i_first_place < 9 {
            let new_i = format!("{}{}", i, i_first_place + 1).parse().unwrap();
            q.push_back(new_i);
        }
    }
}
