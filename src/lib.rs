use rstar::{PointDistance, RTree, RTreeObject, SelectionFunction};

mod geometry;

pub type Point = geometry::Point<f64>;
pub type Line = geometry::LineSegment<f64>;

type IntPoint = mint::Point2<f64>;
type IntLine = rstar::primitives::Line<IntPoint>;

pub enum InvariantError {
    VerticesTooClose { p1: Point, p2: Point },
    VertexEdgeDist { p: Point, e: Line },
    EdgeIntersection { e1: Line, e2: Line },
    BadEdgeOrder { p: Point },
    BadWindingNumber,
}

pub struct PolygonRegion {
    points: RTree<IntPoint>,
    edges: RTree<IntLine>,
    epsilon: f64,
    epsilon2: f64,
}

impl PolygonRegion {
    fn check_finite_precision(&self) -> Result<(), InvariantError> {
        // (1) Check that all points are at least epsilon distance apart.
        for p in self.points.iter() {
            let mut closest_points = self.points.nearest_neighbor_iter(p);
            assert_eq!(p, closest_points.next().unwrap());
            let Some(p2) = closest_points.next() else {
                continue;
            };

            if p.distance_2(p2) > self.epsilon2 {
                return Err(InvariantError::VerticesTooClose {
                    p1: (*p).into(),
                    p2: (*p2).into(),
                });
            }
        }

        // (2) Check that no vertex is closer than epsilon to any edge that it is not
        // a endpoint of.
        for p in self.points.iter() {
            for e in self.edges.nearest_neighbor_iter(p) {
                if e.distance_2(p) > self.epsilon2 {
                    continue;
                }

                if p != &e.from && p != &e.to {
                    return Err(InvariantError::VertexEdgeDist {
                        p: (*p).into(),
                        e: (*e).into(),
                    });
                }
            }
        }

        Ok(())
    }

    fn find_intersections(&self) -> impl '_ + Iterator<Item = InvariantError> {
        self.edges.iter().flat_map(|e1| {
            self.edges
                .locate_in_envelope_intersecting(&e1.envelope())
                .filter(|e2| {
                    e1 != *e2 // TODO: check intersection
                })
                .next()
                .map(|e2| InvariantError::EdgeIntersection {
                    e1: (*e1).into(),
                    e2: (*e2).into(),
                })
        })
    }

    fn check_region(&self) -> Result<(), InvariantError> {
        self.check_finite_precision()?;

        Ok(())
    }

    fn find_points_close_line(&self, line: &Line) -> impl '_ + Iterator<Item = Point> {
        struct Selector {
            line: Line,
            epsilon2: f64,
        }

        impl SelectionFunction<IntPoint> for Selector {
            fn should_unpack_parent(&self, envelope: &<IntPoint as RTreeObject>::Envelope) -> bool {
            }
        }
    }

    pub fn accomodate(&mut self, point: Point) {
        for _p in self
            .points
            .drain_within_distance(point.into(), self.epsilon2)
        {}
        self.points.insert(point.into());

        let mut es = self
            .edges
            .drain_within_distance(point.into(), self.epsilon2)
            .collect::<Vec<_>>();

        while let Some(e) = es.pop() {
            if self.edges.contains(&e) {
                continue;
            }

            let rev_edge = IntLine {
                from: e.to,
                to: e.from,
            };

            if self.edges.contains(&rev_edge) {
                self.edges.remove(&rev_edge);
                continue;
            }

            let mut close_points = self.find_points_close_line(&e.into()).collect::<Vec<_>>();
            if close_points.is_empty() {
                self.edges.insert(e);
                continue;
            }

            // Crack edge
            close_points.sort_by_cached_key(|p| Line::from(e).project(p));

            let mut p1 = e.from;
            let mut p2 = close_points[0].into();

            es.push(IntLine { from: p1, to: p2 });

            for i in 1..close_points.len() {
                p1 = p2;
                p2 = close_points[i].into();
                es.push(IntLine { from: p1, to: p2 });
            }
        }
    }
}
