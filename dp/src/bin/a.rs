use proconio::input;

fn main() {
    input! {
        N: usize,
        h: [isize; N],
    }

    let mut cost: Vec<isize> = vec![0; N];

    cost[1] = cost[0] + (h[1] - h[0]).abs();

    for i in 2..N {
        cost[i] = std::cmp::min(
            cost[i - 1] + (h[i] - h[i - 1]).abs(),
            cost[i - 2] + (h[i] - h[i - 2]).abs(),
        );
    }

    println!("{}", cost[N - 1]);
}
