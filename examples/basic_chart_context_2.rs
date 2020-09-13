use plotters::prelude::*;

fn main() {
	let drawing_area = BitMapBackend::new("images/2.1.png", (600, 400))
		.into_drawing_area();

	drawing_area.fill(&WHITE).unwrap();
	
	let mut chart = ChartBuilder::on(&drawing_area)
		.build_ranged(0..100, 0..100)
		.unwrap();

	chart.draw_series(
		LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
	).unwrap();
}

#[test]
fn etnry_point() { main(); } 
