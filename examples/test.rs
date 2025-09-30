use polygon_booleans::*;

fn main() {
    let mut region = GeometricRegion::new();

    region.add_point(na::point![1.0, 2.0]);

    let polygon = Polygon::from_outline([
        // ...
    ])
    .expect("invalid outline");

    polygon.inset(0.5);

    polygon.offset(0.5);

    polygon.outline().to_owned();

    for hole in polygon.holes() {}

    let mut multi_polygon = MultiPolygon::new();

    multi_polygon.insert(&polygon);
}

// struct GeometricRegionRef<'a> {
//     region: &'a GeometricRegion,
// }

struct PolygonRegion<'a, PT> {
    polygon_type: PT,
    geometric_region: std::borrow::Cow<'a, GeometricRegion>,
}

struct SimplePolygon;

struct PolygonWithHoles;

type Polygon = PolygonRegion<'static, PolygonWithHoles>;

type PolygonRef<'a> = PolygonRegion<'a, PolygonWithHoles>;
