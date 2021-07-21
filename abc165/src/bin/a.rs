fn main() {}

fn main1() {
    proconio::input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let mut is_ok = false;

    for i in a..=b {
        if i % k == 0 {
            is_ok = true;
        }
    }

    println!("{}", if is_ok { "OK" } else { "NG" });
}

fn main2() {
    proconio::input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let mut n = k;

    let mut is_ok = false;

    loop {
        if a <= n && n <= b {
            is_ok = true;
            break;
        }

        if b < n {
            break;
        }

        n += k;
    }

    println!("{}", if is_ok { "OK" } else { "NG" });
}

fn main3() {
    proconio::input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let is_ok = a / k < b / k || a % k == 0;

    println!("{}", if is_ok { "OK" } else { "NG" });
}
