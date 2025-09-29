use plotters::coord::Shift;
use plotters::prelude::*;
use polygon_booleans::*;

fn draw_region(region: &PolygonRegion, drawing: &DrawingArea<SVGBackend, Shift>) {
    let point_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: true,
        stroke_width: 0,
    };
    let epsilon_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: false,
        stroke_width: 1,
    };
    let edge_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: false,
        stroke_width: 1,
    };
    for point in region.points() {
        drawing.draw(&Circle::new(
            (point.x as i32, point.y as i32),
            3.0,
            point_style,
        ));
        drawing.draw(&Circle::new(
            (point.x as i32, point.y as i32),
            region.epsilon(),
            epsilon_style,
        ));
    }

    for edge in region.edges() {
        drawing.draw(&PathElement::new(
            vec![
                (edge.from.x as i32, edge.from.y as i32),
                (edge.to.x as i32, edge.to.y as i32),
            ],
            edge_style,
        ));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut region = PolygonRegion::new(10.0);

    region.accomodate(Point::new(100.0, 100.0));
    region.accomodate(Point::new(200.0, 200.0));
    region.accomodate(Point::new(300.0, 100.0));

    region.accomodate(Point::new(130.0, 150.0));

    region.add_edge(Line::new(
        Point::new(100.0, 100.0),
        Point::new(200.0, 200.0),
    ));

    // Create a 800*600 bitmap and start drawing
    // let mut backend = BitMapBackend::new("plotters-doc-data/1.png", (300, 200));
    // And if we want SVG backend
    let drawing = SVGBackend::new("output.svg", (800, 600)).into_drawing_area();
    let (left, right) = drawing.split_horizontally(400.0);
    draw_region(&region, &left);

    region.accomodate(Point::new(155.0, 165.0));
    region.add_edge(Line::new(
        Point::new(100.0, 100.0),
        Point::new(350.0, 110.0),
    ));
    draw_region(&region, &right);
    Ok(())
}
