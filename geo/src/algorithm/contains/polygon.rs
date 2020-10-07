use super::Contains;
use crate::relate::Relate;
use crate::{Coordinate, GeoFloat, Line, LineString, MultiPolygon, Point, Polygon};

// ┌─────────────────────────────┐
// │ Implementations for Polygon │
// └─────────────────────────────┘
impl<T> Contains<Coordinate<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn contains(&self, coord: &Coordinate<T>) -> bool {
        use crate::algorithm::coordinate_position::{CoordPos, CoordinatePosition};

        self.coordinate_position(coord) == CoordPos::Inside
    }
}

impl<T> Contains<Point<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn contains(&self, p: &Point<T>) -> bool {
        self.contains(&p.0)
    }
}

impl<T> Contains<Line<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn contains(&self, line: &Line<T>) -> bool {
        self.relate(line).is_contains()
    }
}

impl<T> Contains<Polygon<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn contains(&self, poly: &Polygon<T>) -> bool {
        self.relate(poly).is_contains()
    }
}

impl<T> Contains<LineString<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn contains(&self, linestring: &LineString<T>) -> bool {
        self.relate(linestring).is_contains()
    }
}

// ┌──────────────────────────────────┐
// │ Implementations for MultiPolygon │
// └──────────────────────────────────┘
impl<G, F> Contains<G> for MultiPolygon<F>
where
    F: GeoFloat,
    G: Relate<F, MultiPolygon<F>>,
{
    fn contains(&self, rhs: &G) -> bool {
        rhs.relate(self).is_within()
    }
}
