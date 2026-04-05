mod problems_solutions;

fn main() {
    println!(
        "{}",
        problems_solutions::Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2]
        ])
    );
}
