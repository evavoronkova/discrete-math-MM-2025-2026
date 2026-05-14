use discrete_math_mm_2025_2026::task_1298::Solution;

#[test]
fn test_1() {
    let status = vec![1, 0, 1, 0];
    let candies = vec![7, 5, 4, 100];
    let keys = vec![vec![], vec![], vec![1], vec![]];
    let contained_boxes = vec![vec![1, 2], vec![3], vec![], vec![]];
    let initial_boxes = vec![0];
    let answer = Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes);
    assert_eq!(answer, 16, "Wrong answer");
}

#[test]
fn test_2() {
    let status = vec![1, 0, 0, 0, 0, 0];
    let candies = vec![1, 1, 1, 1, 1, 1];
    let keys = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
    let contained_boxes = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
    let initial_boxes = vec![0];
    let answer = Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes);
    assert_eq!(answer, 6, "Wrong answer");
}
