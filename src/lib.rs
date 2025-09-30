pub mod geometry;
mod polygon_region;

pub use polygon_region::PolygonRegion;

pub struct Polygon {
    region: PolygonRegion,
}
