fn main() {
    let n = 9;
    let s = 0usize;
    let g: Vec<Vec<usize>> = vec![
        vec![1, 2],
        vec![0, 3, 4],
        vec![0, 5],
        vec![1, 6],
        vec![1],
        vec![2, 7, 8],
        vec![3],
        vec![5],
        vec![5],
    ];

    let mut visited = vec![false; n];
    fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, i: usize) {
        visited[i] = true;

        for &j in &g[i] {
            if !visited[j] {
                dfs(g, visited, j);
            }
        }
    }

    dfs(&g, &mut visited, s);
}
