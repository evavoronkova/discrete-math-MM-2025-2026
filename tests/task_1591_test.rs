use discrete_math_mm_2025_2026::task_1591::Solution;

#[test]
fn test_1() {
    let target_grid = vec![
        vec![1, 1, 1, 1],
        vec![1, 2, 2, 1],
        vec![1, 2, 2, 1],
        vec![1, 1, 1, 1],
    ];
    let answer = Solution::is_printable(target_grid);
    assert_eq!(answer, true, "Wrong answer");
}

#[test]
fn test_2() {
    let target_grid = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 3, 3],
        vec![1, 1, 3, 4],
        vec![5, 5, 1, 4],
    ];
    let answer = Solution::is_printable(target_grid);
    assert_eq!(answer, true, "Wrong answer");
}

#[test]
fn test_3() {
    let target_grid = vec![vec![1, 2, 1], vec![2, 1, 2], vec![1, 2, 1]];
    let answer = Solution::is_printable(target_grid);
    assert_eq!(answer, false, "Wrong answer");
}
