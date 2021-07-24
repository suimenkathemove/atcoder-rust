fn main() {
    proconio::input! {
        n: usize,
        q: usize,
    }

    let mut g = vec![vec![false; n]; n];

    for _ in 0..q {
        proconio::input! {
            t: usize,
            mut a: usize,
        }
        a -= 1;

        match t {
            1 => {
                proconio::input! {
                    mut b: usize,
                }
                b -= 1;

                g[a][b] = true;
            }
            2 => {
                for i in 0..n {
                    if g[i][a] {
                        g[a][i] = true;
                    }
                }
            }
            3 => {
                let mut js = vec![];

                for i in 0..n {
                    if g[a][i] {
                        for j in 0..n {
                            if g[i][j] && j != a {
                                js.push(j);
                            }
                        }
                    }
                }

                for j in js {
                    g[a][j] = true;
                }
            }
            _ => panic!(),
        }
    }

    for i in 0..n {
        for j in 0..n {
            if g[i][j] {
                print!("Y");
            } else {
                print!("N");
            }
        }
        println!();
    }
}
