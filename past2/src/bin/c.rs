fn main() {
    proconio::input! {
            n: usize,
            s: [String; n],
    }

    let mut s: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect()).collect();

    for i in (0..n - 1).rev() {
        for j in 1..2 * n - 2 {
            if s[i][j] == '#' && (j - 1..=j + 1).any(|n| s[i + 1][n] == 'X') {
                s[i][j] = 'X'
            }
        }
    }

    let ans: Vec<String> = s.iter().map(|c| c.iter().collect()).collect();

    for a in ans {
        println!("{}", a);
    }
}
