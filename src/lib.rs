use rstar::primitives::Line;
use rstar::{RTree, PointDistance};

type Point = [f32; 2];

pub struct PolygonRegion {
    points: RTree<Point>,
    edges: RTree<Line<Point>>,
    epsilon: f32,
}

impl PolygonRegion {
    fn check_finite_precision(&self) -> bool {
        // (1) Check that all points are at least epsilon distance apart.
        for p in self.points.iter() {
            let mut closest_points = self.points.nearest_neighbor_iter(p);
            assert_eq!(p, closest_points.next().unwrap());
            if p.distance_2(closest_points.next().unwrap()) >= self.epsilon {
                return false
            }
        }

        // (2) Check that no vertex is closer than epsilon to any edge that it is not
        // a endpoint of.
        for p in self.points.iter() {
            for e in self.edges.nearest_neighbor_iter(p) {
                if e.distance_2(p) >= self.epsilon {
                    continue
                }

                if p != &e.from && p != &e.to {
                    return false
                }
            }
        }

        return true
    }

    fn shift_vertex(&mut self, point: &Point) -> Point {
        self.points.nearest_neighbor_iter(point).next()
            .filter(|p| point.distance_2(p) < self.epsilon)
            .unwrap_or(*point)
    }

    fn crack_edge()

    pub fn accomodate(&mut self, point: &Point) -> Point {
    }
}
