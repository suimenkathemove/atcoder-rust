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

fn brute_force() {
    proconio::input! {
        s: String,
    }

    fn is_match(t: &str, s: &str) -> bool {
        if t.len() > s.len() {
            return false;
        }
        for i in 0..=s.len() - t.len() {
            let mut ok = true;

            for j in 0..t.len() {
                let t_char = &t.chars().collect::<Vec<_>>()[j];
                let s_char = &s.chars().collect::<Vec<_>>()[i + j];
                if t_char != s_char && t_char != &'.' {
                    ok = false;
                }
            }

            if ok {
                return true;
            }
        }

        false
    }

    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '.',
    ];

    let mut set: HashSet<String> = HashSet::new();

    for c1 in &chars {
        let t = c1.to_string();
        if is_match(&t, &s) {
            set.insert(t);
        }
    }

    for c1 in &chars {
        for c2 in &chars {
            let t = format!("{}{}", c1, c2);
            if is_match(&t, &s) {
                set.insert(t);
            }
        }
    }

    for c1 in &chars {
        for c2 in &chars {
            for c3 in &chars {
                let t = format!("{}{}{}", c1, c2, c3);
                if is_match(&t, &s) {
                    set.insert(t);
                }
            }
        }
    }

    println!("{}", set.len());
}
