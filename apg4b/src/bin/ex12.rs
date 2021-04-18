fn main() {
    proconio::input! {
        s: String,
    }

    let mut ans = 1;

    for c in s.chars() {
        match c {
            '+' => ans += 1,
            '-' => ans -= 1,
            _ => (),
        }
    }

    println!("{}", ans);
}
