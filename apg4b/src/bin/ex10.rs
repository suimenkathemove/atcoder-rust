fn main() {
    proconio::input! {
        A: u32,
        B: u32,
    }

    let mut a = String::new();
    let mut b = String::new();

    for _ in 0..A {
        a += "]";
    }
    for _ in 0..B {
        b += "]";
    }

    println!("A:{}", a);
    println!("B:{}", b);
}
