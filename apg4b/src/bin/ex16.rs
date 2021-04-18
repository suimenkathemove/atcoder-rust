fn main() {
    proconio::input! {
        a: [i32; 5usize],
    }

    let mut ans = "NO";

    for i in 0..a.len() - 1 {
        if a[i] == a[i + 1] {
            ans = "YES"
        }
    }

    println!("{}", ans);
}
