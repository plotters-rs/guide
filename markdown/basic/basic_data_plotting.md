# Basic data plotting

In this section, let's use Plotters to produce different types of Plotting. Generally speaking, the API `ChartContext::draw_series` provides the functionality to draw any types of chart. In the following parts, let's discuss how to use it to render different types of plots.

## Line series

The following code demonstrate how to draw a line series with Plotters

```rust
use plotters::prelude::*;

fn main() {
  let root_area = BitMapBackend::new("images/2.5.png", (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Line Plot Demo", ("sans-serif", 40))
    .build_cartesian_2d(-10..10, 0..100)
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

The following code demonstrate how we can crate a scatter plot and use different pointing elements. In the example, we use `Circle` and `TriangleMarker` pointing element for two different series.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.6.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..50, -10..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        DATA1.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap();

    ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();
}
const DATA1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
const DATA2: [(i32, i32); 30] = [(1, 22), (0, 22), (1, 20), (2, 24), (4, 26), (6, 24), (5, 27), (6, 27), (7, 27), (8, 30), (10, 30), (10, 33), (12, 34), (13, 31), (15, 35), (14, 33), (17, 36), (16, 35), (17, 39), (19, 38), (21, 38), (22, 39), (23, 43), (24, 44), (24, 46), (26, 47), (27, 48), (26, 49), (28, 47), (28, 50)];
```

And this will produce the following image.

![2.6.png](../../images/2.6.png)

## Area chart

The following demo demonstrate how we can draw an area chart.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.7.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

  let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(
      AreaSeries::new(
        (0..).zip(data.iter().map(|x| *x)), // The data iter
        0,                                  // Baseline
        &RED.mix(0.2) // Make the series opac
      ).border_style(&RED) // Make a brighter border
    )
    .unwrap();
}
```

Result image:

![2.7.png](../../images/2.7.png)

## Histogram

In practice, the histogram can be two things:

1. A bar plot
2. Or a bar plot that shows the distribution of values

For a bar plot, we can simply create with a iterator that yields a series of rectangle. The following code demonstrates how. The function `Rectangle::margin` is used to set a pixel based margin for the rectangle element.

One note here is we used tweaked the coordinate a little bit, we make the X coordinate segmented, so that the axis labels presents in the middle of the value segments. In plotters this is called a coordinate combinator, we are going to discuss the combinators in detail in the next chapter.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.8.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("sans-serif", 40))
        .build_cartesian_2d((0..10).into_segmented(), 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
}
```

Result image:

![2.8.png](../../images/2.8.png)

Similarly, the following code draws a vertical bar chart.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.9.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..50, (0..10).into_segmented())
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(y, x)| {
        let mut bar = Rectangle::new([
            (0, SegmentValue::Exact(y)), 
            (*x, SegmentValue::Exact(y + 1))
        ], GREEN.filled());
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
    .unwrap();
}
```

Result image:

![2.9.png](../../images/2.9.png)

For the second type of histogram, there's a `Histogram` series type is defined for this purpose.

### Visualize distribution

Now we are going to demonstrate how we can use the `Histogram` series to visualize the distribution of the input data. 

```rust
use plotters::prelude::*;

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let root_area = BitMapBackend::new("images/2.13.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Prime Distribution", ("sans-serif", 40))
        .build_cartesian_2d([true, false].into_segmented(), 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let prim:Vec<_> = (2..50).map(is_prime).collect();

    ctx.draw_series(
        Histogram::vertical(&ctx)
        .margin(100)
        .data(prim.iter().map(|x| (x, 1)))
    ).unwrap();
}
```

![2.13.png](../../images/2.13.png)

## Time Series Chart

In theory Plotters supports any data type to be axis. The only requirement is to implement the
axis mapping traits. By default, Plotters has built-in implementation of axis traits for date and
time types. To make a time series chart, you should first import the chrono crate and define a time
range as axis.
The following example shows how we can plot a time series data.

```rust
use plotters::prelude::*;
use chrono::{Utc, TimeZone};

fn main() {
  let root_area = BitMapBackend::new("images/2.11.png", (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let start_date = Utc.ymd(2019, 10, 1);
  let end_date = Utc.ymd(2019, 10, 18);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("MSFT daily close price", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 130.0..145.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new(
            (0..).zip(DATA.iter()).map(|(idx, price)| {
                let day = (idx / 5) * 7 + idx % 5 + 1;
                let date = Utc.ymd(2019,10, day);
                (date, *price)
            }),
            &BLUE,
        )
    ).unwrap();

}
const DATA: [f64; 14] = [ 137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.10, 138.24, 135.67, 137.12, 138.12];
```

Result image:

![2.11.png](../../images/2.11.png)

## Customized series

Plotters allows you draw arbitrary types of series, even the one isn't built into the Plotters crate. Plotters uses a really simple abstraction for a data series: An iterator of drawable elements. Thus if you can make your own series an iterator of drawable element, it's a valid data series and can be
draw on a figure.

## Multiple Data Series

By calling `draw_series` multiple time, Plotters is able to produce the multiple series plot. Thus, we don't limit the developer's ability to put different types of plot series onto the same plot. The following example shows plotting a histogram along with a line plot.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.10.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Histo + Line", ("sans-serif", 40))
        .build_cartesian_2d(0..10, 0..80)
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
```

Result image:

![2.10.png](../../images/2.10.png)

## Legend

Plotters allows user add legend on the figure. Specifically, Plotters called the it "series label". When you call `ChartContext::draw_series`, a result type that carries a handle to a series annotation is returned and you can use it to add a series label to that. After you complete the data plotting, `ChartContext::configure_series_label` can be called to configure and draw the collections of series label. The following example demonstrate how.

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.12.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Legend", ("sans-serif", 40))
        .build_cartesian_2d(-4.0..4.0, -1.2..1.2)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let x_kps: Vec<_> = (-80..80).map(|x| x as f64 / 20.0).collect();
    ctx.draw_series(LineSeries::new(x_kps.iter().map(|x| (*x, x.sin())), &RED))
        .unwrap()
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(x_kps.iter().map(|x| (*x, x.cos())), &BLUE))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}
```

Result image:

![2.12.png](../../images/2.12.png)
