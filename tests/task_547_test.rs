use discrete_math_mm_2025_2026::task_547::Solution;

#[test]
fn test_1() {
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let answer = Solution::find_circle_num(is_connected);
    assert_eq!(answer, 2, "Wrong answer");
}

#[test]
fn test_2() {
    let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let answer = Solution::find_circle_num(is_connected);
    assert_eq!(answer, 3, "Wrong answer");
}
