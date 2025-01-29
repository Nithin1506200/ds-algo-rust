pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];
    for (indx, i) in graph.iter().enumerate() {
        if i.len() == 0 {
            ans.push(indx as i32);
        }
        for j in i {
            if graph[*j as usize].len() == 0 {
                ans.push(indx as i32);
                break;
            }
        }
    }
    ans
}
#[test]
fn test() {
    assert_eq!(
        eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![]
        ]),
        vec![2, 4, 5, 6]
    )
}
