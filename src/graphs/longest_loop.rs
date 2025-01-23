// fn dfs(
//     u: i32,
//     edges: &Vec<i32>,
//     visited: &mut Vec<bool>,
//     dist: &mut Vec<i32>,
//     in_recursion: &mut Vec<bool>,
//     r: &mut i32,
// ) {
//     if u != -1 {
//         visited[u as usize] = true;
//         in_recursion[u as usize] = true;
//         let mut v = edges[u];
//         if v != -1 && !visited[v as usize] {
//             *dist[v] = dist[u] + 1;
//             dfs(v, edges, vector, dist, in_recursion, r);
//         } else if v != -1 && in_recursion[v] {
//             *result = result.max(dist[u] - dist[v] + 1);
//         }
//         *in_recursion[u] = false;
//     }
// }
// pub fn longest_cycle(edges: Vec<i32>) -> i32 {
//     let n: i32 = edges.length() as i32;
//     let mut visited: Vec<bool> = vec![n; false];
//     let mut dist: Vec<i32> = vec![n; 1];
//     let mut in_recursion: Vec<bool> = vec![n, false];
//     let mut result = 0;

//     for i in 0..n {
//         if (!visited[i]) {
//             dfs(
//                 i,
//                 &edges,
//                 &mut vusited,
//                 &mut dist,
//                 &mut in_recursion,
//                 &mut r,
//             );
//         }
//     }
//     result
// }
