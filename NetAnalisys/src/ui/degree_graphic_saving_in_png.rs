use plotters::prelude::*;

pub fn save_graph_plotters(
    data: &[(f32, f32)],
    name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Ok(());
    }

    let x_min = data.first().unwrap().0;
    let x_max = data.last().unwrap().0;

    let (y_min, y_max) = data
        .iter()
        .map(|(_, y)| *y)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(mn, mx), y| {
            (mn.min(y), mx.max(y))
        });

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
