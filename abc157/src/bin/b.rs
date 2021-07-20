fn main() {
    proconio::input! {
        a: [[i32; 3]; 3],
        n: usize,
        b: [i32; n],
    }

    let mut checked = [[false; 3]; 3];

    for &bb in &b {
        for i in 0..3 {
            for j in 0..3 {
                if bb == a[i][j] {
                    checked[i][j] = true;
                }
            }
        }
    }

    let mut is_bingo = false;

    for i in 0..3 {
        if checked[i].iter().all(|&c| c) {
            is_bingo = true;
        }
    }

    for i in 0..3 {
        if checked.iter().all(|&c| c[i]) {
            is_bingo = true
        }
    }

    if (0..3).all(|n| checked[n][n]) {
        is_bingo = true
    }

    if (0..3).all(|n| checked[n][2 - n]) {
        is_bingo = true;
    }

    println!("{}", if is_bingo { "Yes" } else { "No" });
}
