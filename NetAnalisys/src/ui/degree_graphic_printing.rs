use textplots::{Chart, Plot, Shape};

pub fn print_graph(data: Vec<(f32, f32)>) {
    if data.is_empty() {
        return;
    }

    let x_min = data.first().unwrap().0;
    let x_max = data.last().unwrap().0;
    println!("range X: [{:.2}, {:.2}]", x_min, x_max);
    Chart::new(180, 60, x_min, x_max)
        .lineplot(&Shape::Lines(&data))
        .display();
}