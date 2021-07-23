fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        q: usize,
        uv: [(usize, usize); m],
        mut c: [usize; n],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in uv {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    for _ in 0..q {
        proconio::input! {
            t: usize,
            mut x: usize,
        }
        x -= 1;

        println!("{}", c[x]);

        match t {
            1 => {
                for i in &g[x] {
                    c[*i] = c[x];
                }
            }
            2 => {
                proconio::input! {
                    y: usize,
                }

                c[x] = y;
            }
            _ => panic!(),
        }
    }
}
