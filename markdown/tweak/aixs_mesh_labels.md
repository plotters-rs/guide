# Axis, mesh and label tweaks

`ChartBuilder`'s `build_cartesian_2d()` returns a ChartContext. The ChartContext gives you the ability to configure the mesh and axis of your plots using `configure_mesh`. For all implementations, see [the docs](https://docs.rs/plotters/latest/plotters/chart/struct.MeshStyle.html). 

## Lables

### Label formatters

Default labels for the axis are the elements of the ranges given as parameters to `ChartBuilder`'s `build_cartesian_2d()` or `build_cartesian_3d()`. To customize the labels, you need to provide a formatting function.

```rust
use plotters::prelude::*;
fn main() {
    let root = BitMapBackend::new("images/label-formatters.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .build_cartesian_2d(0..5, 0..5)
        .unwrap();
    
    let x_labels = ["Zero", "One", "Two", "Three", "Four", "Five"];

    chart
        .configure_mesh()
        .x_label_formatter(&|x| x_labels[*x as usize].to_string())
        .draw()
        .unwrap();
}
```
![label-formatters](../../images/label-formatters.png)

### Segmented values

If you want custom labels for your segmented values, you could provide a formatting function like: 

```rust
use plotters::prelude::*;
fn main() {
    let root = BitMapBackend::new("images/label-formatters-segment-value.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .build_cartesian_2d((0..5).into_segmented(), 0..5)
        .unwrap();

    let x_labels = ["Zero", "One", "Two", "Three", "Four", "Five"];

    chart 
        .configure_mesh()
        .x_label_formatter(&|x| match x {
            SegmentValue::CenterOf(a) | SegmentValue::Exact(a) => x_labels[*a as usize].to_string(),
            _a => "".to_string(),
        })
        .draw()
        .unwrap();

    let values = [1, 0, 3, 5, 2, 3];

    // Drawing the actual bars as well in order for the labels to make more sense: 
    chart
        .draw_series((0..).zip(values.iter()).map(|(x, y)| {
            let x0 = SegmentValue::Exact(x);
            let x1 = SegmentValue::Exact(x + 1);
            let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
            bar.set_margin(0, 0, 5, 5);
            bar
        }))
        .unwrap();
}
```
![label-formatters-segment-value](../../images/label-formatters-segment-value.png)

## TODO:

- Adjust number of tick marks on axis
- Changing axis, labels and mesh styles
- Partially visible axis
- Inward labels
- Log scale axis

