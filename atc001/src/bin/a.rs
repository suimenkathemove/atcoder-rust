use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
        c: [String; H],
    }

    let (s, g) = {
        let mut s = (0, 0);
        let mut g = (0, 0);

        for i in 0..H {
            for j in 0..W {
                let char = c[i].chars().nth(j).unwrap();
                match char {
                    's' => s = (i, j),
                    'g' => g = (i, j),
                    _ => {}
                }
            }
        }

        (s, g)
    };

    let mut visited = vec![vec![false; W]; H];

    fn dfs(
        visited: &mut Vec<Vec<bool>>,
        &(i, j): &(usize, usize),
        H: usize,
        W: usize,
        c: &[String],
    ) {
        visited[i][j] = true;

        let all_sides: Vec<(usize, usize)> = {
            let mut all_sides = vec![];

            if i > 0 {
                all_sides.push((i - 1, j));
            }
            if i < H - 1 {
                all_sides.push((i + 1, j));
            }
            if j > 0 {
                all_sides.push((i, j - 1));
            }
            if j < W - 1 {
                all_sides.push((i, j + 1));
            }

            all_sides
        };

        for &(i2, j2) in &all_sides {
            let char = c[i2].chars().nth(j2).unwrap();

            if char == '#' {
                continue;
            }

            if !visited[i2][j2] {
                dfs(visited, &(i2, j2), H, W, c);
            }
        }
    }

    dfs(&mut visited, &s, H, W, &c);

    println!("{}", if visited[g.0][g.1] { "Yes" } else { "No" });
}
