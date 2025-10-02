pub mod geometry;
mod polygon_region;

use nalgebra as na;

// pub use polygon_region::PolygonRegion;

struct PolygonRegion<T: na::Scalar> {
    points: Vec<na::Point2<T>>,
    edges: Vec<EdgeEntry>,
    polygons: Vec<PolygonEntry>,
    holes: Vec<HoleEntry>,
}

pub struct VertexRef(usize);

pub struct EdgeRef(usize);

pub struct PolygonRef(usize);

pub struct HoleRef(usize);

struct EdgeEntry {
    origin: VertexRef,
    prev: EdgeRef,
    next: EdgeRef,
    inside: PolygonRef,
    outside: HoleRef,
}

struct PolygonEntry {
    first_hole: Option<HoleRef>,
    first_edge: EdgeRef,
}

struct HoleEntry {
    polygon: PolygonRef,
    next: HoleRef,
    prev: HoleRef,
    first_edge: EdgeRef,
}

pub struct Polyline<T: na::Scalar> {
    points: Vec<na::Point2<T>>,
}

pub struct SimplePolygon<T: na::Scalar> {
    polyline: Polyline<T>,
}

pub struct Polygon<T: na::Scalar> {
    outline: Polyline<T>,
    holes: Vec<Polyline<T>>,
}

pub struct MultiPolygon<T: na::Scalar> {
    polygons: Vec<Polygon<T>>,
}

struct OutlineEntry {
    neighbour: usize,
    first_hole: Option<usize>,
    first_edge: usize,
}
