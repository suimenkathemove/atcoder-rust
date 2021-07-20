fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
    }

    let mut sum = 0;
    for i in &a {
        sum += i;
    }

    let average = sum / (n as i32);

    for i in &a {
        println!("{}", (average - i).abs());
    }
}
