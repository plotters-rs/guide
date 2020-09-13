use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.10.png", (600, 400))
		.into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Histo + Line", ("Arial", 40))
        .build_ranged(0..10, 0..80)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21];

    // Draw the histogram
    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let mut bar = Rectangle::new([(x, 0), (x + 1, *y)], GREEN.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();

    // Draw the line series
    ctx.draw_series(LineSeries::new(
        (0..).zip(data.iter()).map(|(x, y)| (x, *y + 30)),
        &BLUE,
    ))
    .unwrap();
}

#[test]
fn etnry_point() { main(); } 
