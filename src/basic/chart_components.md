# Draw figure components

For most of the time, the chart should have many components, such as the axis,
the mesh grid, etc. The `ChartContext` type is able to draw those component automatically.

## Adding mesh to the chart

The following code demostrate how we can use `ChartContext::configure_mesh` to add a mesh
to the chart.

```rust
use plotters::prelude::*;

fn main() {
	let root_drawing_area = BitMapBackend::new("images/2.2.png", (600, 400))
		.into_drawing_area();

	root_drawing_area.fill(&WHITE).unwrap();

	let mut ctx = ChartBuilder::on(&root_drawing_area)
		.build_ranged(0..100, 0..100)
		.unwrap();

	ctx.configure_mesh().draw().unwrap();

}
```

And this code should produce the following result.

![example-2-2](../../images/2.2.png)
