use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        R: usize,
        C: usize,
        mut s: (usize, usize),
        mut g: (usize, usize),
        c: [String; R],
    }
    s = (s.0 - 1, s.1 - 1);
    g = (g.0 - 1, g.1 - 1);

    let mut dist = vec![vec![-1; C]; R];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    dist[s.0][s.1] = 0;
    q.push_back(s);

    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();

        let mut all_sides: Vec<(usize, usize)> = vec![];
        if i > 0 {
            all_sides.push((i - 1, j));
        }
        if i < R {
            all_sides.push((i + 1, j))
        }
        if j > 0 {
            all_sides.push((i, j - 1))
        }
        if j < C {
            all_sides.push((i, j + 1))
        }

        for &(i2, j2) in &all_sides {
            if c[i2].chars().nth(j2).unwrap() == '#' {
                continue;
            }

            if dist[i2][j2] == -1 {
                dist[i2][j2] = dist[i][j] + 1;

                if i2 == g.0 && j2 == g.1 {
                    break;
                }

                q.push_back((i2, j2));
            }
        }
    }

    println!("{}", dist[g.0][g.1]);
}
