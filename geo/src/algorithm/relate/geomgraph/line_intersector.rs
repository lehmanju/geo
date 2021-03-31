pub(crate) use crate::algorithm::line_intersection::LineIntersection;
use crate::{Coordinate, GeoNum, Line};

pub(crate) trait LineIntersector<F: GeoNum> {
    fn compute_intersection(&mut self, l1: Line<F>, l2: Line<F>) -> Option<LineIntersection<F>>;
}
