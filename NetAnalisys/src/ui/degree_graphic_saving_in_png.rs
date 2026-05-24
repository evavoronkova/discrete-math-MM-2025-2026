use plotters::prelude::*;

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

pub fn save_graph_plotters(
    data: &[(f32, f32)],
    name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (x_min, x_max, y_min, y_max) = match sanitize_bounds(data) {
        Some(b) => b,
        None => return Ok(()),
    };

    let name = format!("{}.png", name.unwrap_or("graph"));
    let root = BitMapBackend::new(&name, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Degree Distribution", ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Degree")
        .y_desc("Percent")
        .draw()?;

    chart.draw_series(LineSeries::new(data.to_vec(), &BLUE))?;

    chart.draw_series(
        data.iter()
            .map(|(x, y)| Circle::new((*x, *y), 3, RED.filled())),
    )?;

    root.present()?;

    println!("График сохранён в {}", name);

    Ok(())
}
