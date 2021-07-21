fn main() {
    proconio::input! {
        n: usize,
    }

    let mut num = n % 9;
    if num == 0 {
        num = 9;
    }

    let digit = (n as f64 / 9.).ceil() as i32;

    let ans = (0..digit)
        .map(|_| num.to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("{}", ans);
}

fn brute_force() {
    proconio::input! {
        n: usize,
    }

    let mut count = 0;

    let mut ans = 0;

    for i in 1.. {
        let mut v: Vec<char> = i.to_string().chars().collect();
        v.dedup();
        if v.len() == 1 {
            count += 1;
            if count >= n {
                ans = i;
                break;
            }
        }
    }

    println!("{}", ans);
}
