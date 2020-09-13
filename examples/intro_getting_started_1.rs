use plotters::prelude::*;

fn main() {
	let root_drawing_area = BitMapBackend::new("result.png", (1024, 768))
		.into_drawing_area();

	root_drawing_area.fill(&WHITE);

	let mut chart = ChartBuilder::on(&root_drawing_area)
		.build_ranged(-3.14..3.14, -1.2..1.2)
		.unwrap();

	chart.draw_series(LineSeries::new(
		(-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
		&RED
	)).unwrap();
}

#[test]
fn etnry_point() { main(); } 
