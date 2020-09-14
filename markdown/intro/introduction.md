# Plotters Developer Guide

Plotters is a drawing library aimed to expedite the production of high-quality data visualization in Rust.

Plotters has a high-level, easy-to-use API for data visualization use case. It's designed to be highly flexible and extensible. As a drawing library, Plotters doesn't limit user to particular types of visualization. You can easily create your own data visulization with Plotters API.

Plotters can target wide range of backends and supports both static and realtime rendering. Plotters fits many different use cases, include:

- Producing publication figure,
- Realtime data visualization on an embeded device,
- Plot widget for GUI frameworks,
- Web pages,
- And more.

## Source code in this book

Please go to [Github repository](https://github.com/plotters-rs/guide). All the sample code is under `examples`
directory. 
To try it yourself, please clone the book repo and use the following command to run all examples:

```bash
cargo test 
```

And all the example code will be automatically extracted to the `src` directory.

### FAQ List

1. Why the example just exits without any figure popping up?

    You should be table to find the output under `images` directory under the user's guide repository. 
    The filename for the output is the defined in the example code.

## API Docs

This book is a developer's guide for Plotters. You may also want the API reference, please go to [docs.rs](https://docs.rs/plotters).

## Interactive Tutorial

There's an interactive tutorial with Jupyter notebook + excvr availible, feel free to check the [statically rendered notebook](https://plotters-rs.github.io/plotters-doc-data/evcxr-jupyter-integration.html)
and follow the instruction to setup the interactive tutorial on your local.

## License and Source Code

Plotters is a free and open source software under [MIT license](https://github.com/38/plotters/blob/master/LICENSE). 

You can find the source code on our [Github repository](https://github.com/38/plotters). 
Currently Plotters is actively maintained and envolving very fast. Any involement, 
including PR, suggestion, idea and issue is warmly welcomed.
