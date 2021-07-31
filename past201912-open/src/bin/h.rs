use std::cmp::min;

fn main() {
    proconio::input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
    }

    let mut sell = 0;

    let mut set = 0;
    let mut all = 0;

    let mut min_set_c = 1_000_000_000;
    let mut min_all_c = 1_000_000_000;

    for i in 0..n {
        if i % 2 == 0 {
            min_set_c = min(c[i], min_set_c);
        } else {
            min_all_c = min(c[i], min_all_c);
        }
    }

    for _ in 0..q {
        proconio::input! {
            t: usize,
        }

        match t {
            1 => {
                proconio::input! {
                    mut x: usize,
                    a: usize,
                }
                x -= 1;

                let x_c = if x % 2 == 0 {
                    c[x] - all - set
                } else {
                    c[x] - all
                };

                if x_c >= a {
                    c[x] -= a;
                    sell += a;

                    if x % 2 == 0 {
                        min_set_c = min(c[x], min_set_c);
                    } else {
                        min_all_c = min(c[x], min_all_c);
                    }
                }
            }
            2 => {
                proconio::input! {
                    a: usize,
                }

                if min_set_c - set - all >= a {
                    set += a;
                }
            }
            3 => {
                proconio::input! {
                    a: usize,
                }

                if min(min_set_c - set - all, min_all_c - all) >= a {
                    all += a;
                }
            }
            _ => panic!(),
        }
    }

    for i in 0..n {
        if i % 2 == 0 {
            sell += set;
        }
    }

    sell += all * n;

    println!("{}", sell);
}
