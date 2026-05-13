use discrete_math_mm_2025_2026::task_3286::Solution;

#[test]
fn test_1() {
    let grid = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 0],
    ];
    let health = 1;
    let answer = Solution::find_safe_walk(grid, health);
    assert_eq!(answer, true, "Wrong answer");
}

#[test]
fn test_2() {
    let grid = vec![
        vec![0, 1, 1, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1],
        vec![0, 0, 1, 0, 1, 0],
    ];
    let health = 3;
    let answer = Solution::find_safe_walk(grid, health);
    assert_eq!(answer, false, "Wrong answer");
}

#[test]
fn test_3() {
    let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let health = 5;
    let answer = Solution::find_safe_walk(grid, health);
    assert_eq!(answer, true, "Wrong answer");
}
