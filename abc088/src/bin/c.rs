fn main() {
    proconio::input! {
        c: [[i32; 3]; 3]
    }

    let mut ok = true;

    let mut v: Vec<_> = (0..3).map(|i| c[i][0] - c[i][1]).collect();
    v.dedup();
    if v.len() != 1 {
        ok = false
    }

    let mut v: Vec<_> = (0..3).map(|i| c[i][1] - c[i][2]).collect();
    v.dedup();
    if v.len() != 1 {
        ok = false
    }

    let mut v: Vec<_> = (0..3).map(|i| c[0][i] - c[1][i]).collect();
    v.dedup();
    if v.len() != 1 {
        ok = false
    }

    let mut v: Vec<_> = (0..3).map(|i| c[1][i] - c[2][i]).collect();
    v.dedup();
    if v.len() != 1 {
        ok = false
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
