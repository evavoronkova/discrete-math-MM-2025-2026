mod problems_solutions;

fn main() {
    println!(
        "first {}",
        problems_solutions::Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2]
        ])
    );
    println!(
        "2.1 {}",
        problems_solutions::Solution::find_safe_walk(
            vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ],
            1
        )
    );
    println!(
        "2.2 {}",
        problems_solutions::Solution::find_safe_walk(vec![vec![1, 1, 1, 1]], 4)
    );
}
