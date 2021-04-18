fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
    }

    fn sum(vec: Vec<i32>) -> i32 {
        let mut sum = 0;
        for num in &vec {
            sum += num;
        }
        sum
    }

    println!("{}", sum(a) * sum(b) * sum(c))
}
