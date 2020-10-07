use super::Contains;
use crate::*;

// ┌──────────────────────────────┐
// │ Implementations for Geometry │
// └──────────────────────────────┘

impl<T> Contains<Coordinate<T>> for Geometry<T>
where
    T: GeoFloat,
{
    fn contains(&self, coord: &Coordinate<T>) -> bool {
        self.contains(&Point::from(*coord))
    }
}

impl<T> Contains<Point<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn contains(&self, point: &Point<T>) -> bool;
    }
}

// ┌────────────────────────────────────────┐
// │ Implementations for GeometryCollection │
// └────────────────────────────────────────┘

impl<T> Contains<Coordinate<T>> for GeometryCollection<T>
where
    T: GeoFloat,
{
    fn contains(&self, coord: &Coordinate<T>) -> bool {
        self.iter().any(|geometry| geometry.contains(coord))
    }
}

impl<T> Contains<Point<T>> for GeometryCollection<T>
where
    T: GeoFloat,
{
    fn contains(&self, point: &Point<T>) -> bool {
        self.contains(&point.0)
    }
}
