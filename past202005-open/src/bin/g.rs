use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        N: usize,
        X: isize,
        Y: isize,
        xy: [(isize, isize); N],
    }

    const OFFSET: usize = 201;
    const MAX_INDEX: usize = (201 + OFFSET) as usize;
    const MAX: usize = (MAX_INDEX + 1) as usize;

    let X = (X + OFFSET as isize) as usize;
    let Y = (Y + OFFSET as isize) as usize;

    let xy = {
        let mut v = [[false; MAX]; MAX];

        for n in 0..N {
            let (i, j) = xy[n];
            let i = (i + OFFSET as isize) as usize;
            let j = (j + OFFSET as isize) as usize;

            v[i][j] = true;
        }

        v
    };

    let mut d = [[-1; MAX]; MAX];
    let mut q = VecDeque::new();

    d[OFFSET][OFFSET] = 0;
    q.push_back((OFFSET, OFFSET));

    while let Some((i, j)) = q.pop_front() {
        let v = {
            let mut v = vec![];

            if i > 0 {
                v.push((i - 1, j));

                if j < MAX_INDEX {
                    v.push((i - 1, j + 1));
                }
            }
            if i < MAX_INDEX {
                v.push((i + 1, j));

                if j < MAX_INDEX {
                    v.push((i + 1, j + 1));
                }
            }
            if j > 0 {
                v.push((i, j - 1));
            }
            if j < MAX_INDEX {
                v.push((i, j + 1));
            }

            v
        };

        for (i2, j2) in v {
            if xy[i2][j2] {
                continue;
            }

            if d[i2][j2] == -1 {
                d[i2][j2] = d[i][j] + 1;

                if !(i2 == X && j2 == Y) {
                    q.push_back((i2, j2));
                }
            }
        }
    }

    println!("{}", d[X][Y]);
}
