use discrete_math_mm_2025_2026::task_1462::Solution;

#[test]
fn test_1() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    let queries = vec![vec![0, 1], vec![1, 0]];
    let answer = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
    assert_eq!(answer, vec![false, true], "Wrong answer");
}

#[test]
fn test_2() {
    let num_courses = 2;
    let prerequisites = vec![];
    let queries = vec![vec![1, 0], vec![0, 1]];
    let answer = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
    assert_eq!(answer, vec![false, false], "Wrong answer");
}

#[test]
fn test_3() {
    let num_courses = 3;
    let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
    let queries = vec![vec![1, 0], vec![1, 2]];
    let answer = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
    assert_eq!(answer, vec![true, true], "Wrong answer");
}
