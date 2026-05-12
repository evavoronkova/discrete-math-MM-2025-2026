use discrete_math_mm_2025_2026::task_851::Solution;

#[test]
fn test_1() {
    let richer = vec![
        vec![1, 0],
        vec![2, 1],
        vec![3, 1],
        vec![3, 7],
        vec![4, 3],
        vec![5, 3],
        vec![6, 3],
    ];
    let quiet = vec![3, 2, 5, 4, 6, 1, 7, 0];
    let answer = Solution::loud_and_rich(richer, quiet);
    assert_eq!(answer, vec![5, 5, 2, 5, 4, 5, 6, 7], "Wrong answer");
}

#[test]
fn test_2() {
    let richer = vec![];
    let quiet = vec![0];
    let answer = Solution::loud_and_rich(richer, quiet);
    assert_eq!(answer, vec![0], "Wrong answer");
}
