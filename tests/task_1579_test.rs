use discrete_math_mm_2025_2026::task_1579::Solution;

#[test]
fn test_1() {
    let n = 4;
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4],
    ];
    let answer = Solution::max_num_edges_to_remove(n, edges);
    assert_eq!(answer, 2, "Wrong answer");
}

#[test]
fn test_2() {
    let n = 4;
    let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
    let answer = Solution::max_num_edges_to_remove(n, edges);
    assert_eq!(answer, 0, "Wrong answer");
}

#[test]
fn test_3() {
    let n = 4;
    let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];
    let answer = Solution::max_num_edges_to_remove(n, edges);
    assert_eq!(answer, -1, "Wrong answer");
}
