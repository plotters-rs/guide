use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.7.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("Arial", 40))
        .build_ranged(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter().map(|x| *x)), // The data iter
            0,                                  // Baseline
            &RED.mix(0.2),                      // Make the series opac
        )
        .border_style(&RED), // Make a brighter border
    )
    .unwrap();
}
