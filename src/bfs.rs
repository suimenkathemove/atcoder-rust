use std::collections::VecDeque;

fn main() {
    let n = 9;
    let s = 0;
    let g = vec![
        vec![1, 5],
        vec![0, 2, 4],
        vec![1, 3],
        vec![2],
        vec![1],
        vec![0, 6],
        vec![5, 7, 8],
        vec![6],
        vec![6],
    ];

    let mut visited = vec![false; n];
    let mut q: VecDeque<usize> = VecDeque::new();

    q.push_back(s);
    visited[s] = true;

    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        for j in &g[i] {
            if !visited[*j] {
                q.push_back(*j);
                visited[*j] = true;
            }
        }
    }
}
