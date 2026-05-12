use discrete_math_mm_2025_2026::task_3112::Solution;

#[test]
fn test_1() {
    let n = 3;
    let edges = vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]];
    let disappear = vec![1, 1, 5];
    let answer = Solution::minimum_time(n, edges, disappear);
    assert_eq!(answer, vec![0, -1, 4], "Wrong answer");
}

#[test]
fn test_2() {
    let n = 3;
    let edges = vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]];
    let disappear = vec![1, 3, 5];
    let answer = Solution::minimum_time(n, edges, disappear);
    assert_eq!(answer, vec![0, 2, 3], "Wrong answer");
}

#[test]
fn test_3() {
    let n = 2;
    let edges = vec![vec![0, 1, 1]];
    let disappear = vec![1, 1];
    let answer = Solution::minimum_time(n, edges, disappear);
    assert_eq!(answer, vec![0, -1], "Wrong answer");
}
