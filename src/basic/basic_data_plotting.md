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

The following demo demostrate how we can draw an area chart.

```rust
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
			(0..).zip(data.into_iter()), // The data iter
			0,   // Baseline
			&RED.mix(0.2) // Make the series opac
		).border_style(&RED) // Make a brighter border
    )
    .unwrap();
}
```

Result image:

![2.7.png](../../images/2.7.png)

## Histogram

In practise, the histogram can be two things:

1. A bar plot
2. Or a bar plot that shows the distribution of values

For a bar plot, we can simply create with a iterator that yields a series of
rectangle. The following code demostrates how. The function `Rectangle::margin` 
is used to set a pixel based margin for the rectangle element.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.8.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("Arial", 40))
        .build_ranged(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21];

    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let mut bar = Rectangle::new([(x, 0), (x + 1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
}
```

Result image:

![2.8.png](../../images/2.8.png)

Similary, the following code draws a vertical bar chart.

```rust
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
```

Result image:

![2.9.png](../../images/2.9.png)

For the second type of histogram, there's a `Histograma` series type is defined for this purpose.

## Error bar
## Heatmap
## Customized series
## Mutiple Data Series
## Legend
