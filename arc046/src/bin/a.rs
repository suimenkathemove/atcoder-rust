fn main() {}

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
