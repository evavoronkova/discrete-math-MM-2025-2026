use discrete_math_mm_2025_2026::task_210::Solution;

#[test]
fn test_1() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    let answer = Solution::find_order(num_courses, prerequisites);
    assert_eq!(answer, vec![0, 1], "Wrong answer");
}

#[test]
fn test_2() {
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    let answer = Solution::find_order(num_courses, prerequisites);
    assert_eq!(answer, vec![0, 1, 2, 3], "Wrong answer");
}

#[test]
fn test_3() {
    let num_courses = 1;
    let prerequisites = vec![];
    let answer = Solution::find_order(num_courses, prerequisites);
    assert_eq!(answer, vec![0], "Wrong answer");
}
