use discrete_math_MM_2025_2026::task_886::Solution;

#[test]
fn true_test () {
    let n = 4;
    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
    let answer = Solution::possible_bipartition(n, dislikes);
    assert_eq!(answer, true, "Wrong answer");
}

#[test]
fn false_test_different_groups () {
    let n = 3;
    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let answer = Solution::possible_bipartition(n, dislikes);
    assert_eq!(answer, false, "Wrong answer");
}

#[test]
fn false_test_one_group () {
    let n = 5;
    let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    let answer = Solution::possible_bipartition(n, dislikes);
    assert_eq!(answer, false, "Wrong answer");
}