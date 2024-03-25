use geo::{Point, algorithm::euclidean_distance::EuclideanDistance};
use plotters::prelude::*;
use kdtree::KdTree;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    // Create a vector database
    let mut database: Vec<Point<f64>> = Vec::new();

    // Add some random points to the database
    for _ in 0..100 {
        let x = rng.gen_range(0.0..10.0);
        let y = rng.gen_range(0.0..10.0);
        database.push(Point::new(x, y));
    }

    // Create a KdTree for nearest neighbor search
    let mut kdtree = KdTree::new(2);
    for (i, point) in database.iter().enumerate() {
        kdtree.add([point.x(), point.y()], i)?;
    }

    // Find the nearest point to a given location
    let location = Point::new(5.0, 5.0);
    let nearest = kdtree.nearest(&[location.x(), location.y()], 1, &squared_euclidean)?;
    let nearest_point = database[*nearest[0].1];

    println!("Nearest point to ({}, {}): ({}, {})", location.x(), location.y(), nearest_point.x(), nearest_point.y());

    // Visualize the points and the nearest point
    let root = BitMapBackend::new("points.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Points", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    for point in &database {
        chart.draw_series(PointSeries::of_element(
            [(point.x(), point.y())],
            5,
            ShapeStyle::from(&BLUE).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
            },
        ))?;
    }

    chart.draw_series(PointSeries::of_element(
        [(nearest_point.x(), nearest_point.y())],
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
        },
    ))?;

    // Draw a cross at the location we are searching the nearest points to
    chart.draw_series(PointSeries::of_element(
        [(location.x(), location.y())],
        5,
        ShapeStyle::from(&BLACK).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Cross::new((0, 0), size, style)
        },
    ))?;

    root.present()?;

    // ...

    Ok(())
}

fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b) * (a - b)).sum()
}