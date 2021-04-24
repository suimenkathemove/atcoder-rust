fn main() {
    proconio::input! {
        n: i32,
    }

    let str_n = n.to_string();
    let trimmed_str_n = str_n.trim_end_matches('0');

    println!(
        "{}",
        if trimmed_str_n == trimmed_str_n.chars().rev().collect::<String>() {
            "Yes"
        } else {
            "No"
        }
    );
}
