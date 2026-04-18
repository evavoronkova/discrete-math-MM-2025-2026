mod problems_solutions;

fn main() {
    // println!(
    //     "first {}",
    //     problems_solutions::Solution::remove_stones(vec![
    //         vec![0, 0],
    //         vec![0, 2],
    //         vec![1, 1],
    //         vec![2, 0],
    //         vec![2, 2]
    //     ])
    // );
    // println!(
    //     "2.1 {}",
    //     problems_solutions::Solution::find_safe_walk(
    //         vec![
    //             vec![0, 1, 0, 0, 0],
    //             vec![0, 1, 0, 1, 0],
    //             vec![0, 0, 0, 1, 0]
    //         ],
    //         1
    //     )
    // );
    // println!(
    //     "2.2 {}",
    //     problems_solutions::Solution::find_safe_walk(vec![vec![1, 1, 1, 1]], 4)
    // );
    // Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
    // println!(
    //     "3.1 {}",
    //     problems_solutions::Solution::find_cheapest_price(
    //         4,
    //         vec![
    //             vec![0, 1, 100],
    //             vec![1, 2, 100],
    //             vec![2, 0, 100],
    //             vec![1, 3, 600],
    //             vec![2, 3, 200]
    //         ],
    //         0,
    //         3,
    //         1
    //     )
    // );

    // println!(
    //     "4.1 {:?}",
    //     problems_solutions::Solution::construct_grid_layout(
    //         4,
    //         vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]
    //     )
    // );

    println!(
        "5.1 {}",
        problems_solutions::Solution::minimum_weight(
            6,
            vec![
                vec![0, 2, 2],
                vec![0, 5, 6],
                vec![1, 0, 3],
                vec![1, 4, 5],
                vec![2, 1, 1],
                vec![2, 3, 3],
                vec![2, 4, 4],
                vec![3, 0, 4],
                vec![3, 4, 2],
                vec![4, 5, 1]
            ],
            0,
            1,
            5
        )
    );
}
