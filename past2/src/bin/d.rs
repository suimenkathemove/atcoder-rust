use std::cmp::min;
use std::collections::HashSet;

fn main() {
    proconio::input! {
        s: String,
    }

    let mut set: HashSet<String> = HashSet::new();

    for length in 1..=min(3, s.len()) {
        for start in 0..=s.len() - length {
            let str = s.chars().map(|c| c.to_string()).collect::<Vec<_>>()[start..start + length]
                .join("");

            fn func(current: &str, str: &str, set: &mut HashSet<String>) {
                if current.len() >= str.len() {
                    set.insert(current.to_string());
                    return;
                }

                func(
                    &format!("{}{}", current, str.chars().nth(current.len()).unwrap()),
                    str,
                    set,
                );
                func(&format!("{}{}", current, "."), str, set);
            };

            func("", &str, &mut set);
        }
    }

    println!("{}", set.len());
}
