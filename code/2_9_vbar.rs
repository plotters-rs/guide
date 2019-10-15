use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.9.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("Arial", 40))
        .build_ranged(0..50, 0..10)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21];

    ctx.draw_series((0..).zip(data.iter()).map(|(y, x)| {
        let mut bar = Rectangle::new([(0, y), (*x, y + 1)], GREEN.filled());
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
    .unwrap();
}
