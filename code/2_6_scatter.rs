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

const DATA1: [(i32, i32); 30] = [
    (-3, 1),
    (-2, 3),
    (4, 2),
    (3, 0),
    (6, -5),
    (3, 11),
    (6, 0),
    (2, 14),
    (3, 9),
    (14, 7),
    (8, 11),
    (10, 16),
    (7, 15),
    (13, 8),
    (17, 14),
    (13, 17),
    (19, 11),
    (18, 8),
    (15, 8),
    (23, 23),
    (15, 20),
    (22, 23),
    (22, 21),
    (21, 30),
    (19, 28),
    (22, 23),
    (30, 23),
    (26, 35),
    (33, 19),
    (26, 19),
];

const DATA2: [(i32, i32); 30] = [
    (1, 22),
    (0, 22),
    (1, 20),
    (2, 24),
    (4, 26),
    (6, 24),
    (5, 27),
    (6, 27),
    (7, 27),
    (8, 30),
    (10, 30),
    (10, 33),
    (12, 34),
    (13, 31),
    (15, 35),
    (14, 33),
    (17, 36),
    (16, 35),
    (17, 39),
    (19, 38),
    (21, 38),
    (22, 39),
    (23, 43),
    (24, 44),
    (24, 46),
    (26, 47),
    (27, 48),
    (26, 49),
    (28, 47),
    (28, 50),
];
