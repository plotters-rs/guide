use plotters::prelude::*;

fn main() {
	let drawing_area = BitMapBackend::new("result.pngn", (1024, 768))
		.into_drawing_area();
	
	let _chart = ChartBuilder::on(&drawing_area)
		.build_ranged(0..100, 0..100)
		.unwrap();
}

#[test]
fn etnry_point() { main(); } 
