use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        N: usize,
        K: usize,
        h: [usize; N],
    }

    let mut costs = vec![0; N];

    for i in 0..=K {
        if i > N - 1 {
            continue;
        }

        costs[i] = (h[i] as isize - h[0] as isize).abs();
    }

    for i in K + 1..N {
        for j in 1..=K {
            let new_cost = costs[i - j] + (h[i] as isize - h[i - j] as isize).abs();
            match j {
                1 => costs[i] = new_cost,
                _ => costs[i] = min(costs[i], new_cost),
            }
        }
    }

    println!("{}", costs[N - 1]);
}
