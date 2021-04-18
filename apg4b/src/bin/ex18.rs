fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        data: [[i32; 2usize]; m],
    }

    let mut table = vec![vec!['-'; n]; n];

    for d in &data {
        let i = (d[0] - 1) as usize;
        let j = (d[1] - 1) as usize;

        table[i][j] = 'o';
        table[j][i] = 'x';
    }

    for i in 0..n {
        let mut str = String::new();

        for j in 0..n {
            str += &table[i][j].to_string();
            if j != n - 1 {
                str += " ";
            }
        }

        println!("{}", str);
    }
}
