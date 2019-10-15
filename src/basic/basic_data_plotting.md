# Basic data plotting

In this section, let's use Plotters to produce different types of Plotting.
Generally speaking, the API `ChartContext::draw_series` provides the functionality
to draw any types of chart. In the following parts, let's dicuss how to use it to 
render different types of plots.

## Line series

The following code demostrate how to draw a line series with Plotters

```rust
use plotters::prelude::*;

fn main() {
	let root_area = BitMapBackend::new("images/2.5.png", (600, 400)).into_drawing_area();
	root_area.fill(&WHITE).unwrap();

	let mut ctx = ChartBuilder::on(&root_area)
		.set_label_area_size(LabelAreaPosition::Left, 40)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.caption("Line Plot Demo", ("Arial", 40))
		.build_ranged(-10..10, 0..100)
		.unwrap();

	ctx.configure_mesh().draw().unwrap();

	ctx.draw_series(
		LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)
	).unwrap();
}
```

It should produce the following image

![2.5.png](../../images/2.5.png)

## Scatter Plot

The following code demostrate how we can crate a scatter plot and use different pointing elements.
In the example, we use `Circle` and `TriagnleMarker` pointing element for two different series.

In this example, we assume there are `DATA1` and `DATA2` defined. See the source for the details.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.6.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("Arial", 40))
        .build_ranged(-10..50, -10..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        DATA1
            .into_iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap();

    ctx.draw_series(DATA2.into_iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();
}
```

And this will produce the following image.

![2.6.png](../../images/2.6.png)

## Area chart
## Histogram
## Error bar
## Heatmap
## Customized series
## Mutiple Data Series
## Legend
