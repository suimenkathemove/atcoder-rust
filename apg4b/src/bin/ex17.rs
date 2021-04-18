fn main() {
    proconio::input! {
        n: usize,
        s: i32,
        a: [i32; n],
        p: [i32; n],
    }

    let mut ans = 0;

    for i in &a {
        for j in &p {
            if i + j == s {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
