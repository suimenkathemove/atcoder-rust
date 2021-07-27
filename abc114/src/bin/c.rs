use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        N: usize,
    }

    let mut set = HashSet::new();

    fn dfs(
        current: usize,
        i: usize,
        N: usize,
        set: &mut HashSet<usize>,
        has_3: bool,
        has_5: bool,
        has_7: bool,
    ) {
        let current = format!("{}{}", current, i).parse().unwrap();

        if current > N {
            return;
        }

        if has_3 && has_5 && has_7 {
            set.insert(current);
        }

        dfs(current, 3, N, set, true, has_5, has_7);
        dfs(current, 5, N, set, has_3, true, has_7);
        dfs(current, 7, N, set, has_3, has_5, true);
    }

    dfs(0, 3, N, &mut set, true, false, false);
    dfs(0, 5, N, &mut set, false, true, false);
    dfs(0, 7, N, &mut set, false, false, true);

    println!("{}", set.len());
}
