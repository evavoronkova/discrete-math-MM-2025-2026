use textplots::{Chart, Plot, Shape};

fn sanitize_bounds(data: &[(f32, f32)]) -> Option<(f32, f32, f32, f32)> {
    if data.is_empty() {
        return None;
    }
    let x_min = data.first()?.0;
    let x_max = data.last()?.0;
    if !x_min.is_finite() || !x_max.is_finite() {
        return None;
    }
    let y_vals: Vec<f32> = data.iter().map(|(_, y)| *y).collect();
    let y_min = y_vals.iter().copied().fold(f32::INFINITY, f32::min);
    let y_max = y_vals.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    if !y_min.is_finite() || !y_max.is_finite() {
        return None;
    }
    Some((x_min, x_max, y_min, y_max))
}

pub fn print_graph(data: &[(f32, f32)]) {
    let (x_min, x_max, _y_min, _y_max) = match sanitize_bounds(data) {
        Some(b) => b,
        None => return,
    };
    println!("range X: [{:.2}, {:.2}]", x_min, x_max);
    println!("|{}|", "-".repeat(180));
    Chart::new(180, 60, x_min, x_max)
        .lineplot(&Shape::Lines(&data))
        .display();
    println!("|{}|", "-".repeat(180));
}
