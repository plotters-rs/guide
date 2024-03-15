# Animation and realtime rendering

All the previous examples are of  statically rendered images. In this section, we are going to show how to generate realtime or animated plots. To render a realtime or animated figure, you need to redraw the figure periodically: For an animated plot, you should render each frame, while for a GUI integration, you should handle the redraw request.

The realtime rendering semantics for Plotters is supported by the API `DrawingBackend::present`, which means we tell the drawing backend we have finished drawing the current frame. The following drawing will belong to the next frame. 

At the same time, you need a backend that supports realtime rendering. When GIF support is enabled, the default `BitMapBackend` is able to produce animated images. The following example shows show. 

```rust
use plotters::prelude::*;
fn main() {
    let area = BitMapBackend::gif(
        "images/animated.gif", 
        (320, 100), 
        1_000  /* Each frame show 1s */
    ).unwrap().into_drawing_area();
    for i in 0..=10 {
        area.fill(&WHITE).unwrap();
        area.draw(
            &Text::new(
                format!("{}", 10 - i), 
                (100, 20),
                ("sans-serif", 80)
            )
       ).unwrap();
       area.present().unwrap();
    }
}
```
![animated](../../images/animated.gif)

Similarly when Plotters is integrated to a GUI framework, all you need to do is redraw the figure and call `present` after the plot rendering.
