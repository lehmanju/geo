use crate::GeoNum;

/// Utility functions for working with quadrants of the cartesian plane,
/// which are labeled as follows:
///          (+)
///        NW ┃ NE
///    (-) ━━━╋━━━━ (+)
///        SW ┃ SE
///          (-)
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Quadrant {
    NE,
    NW,
    SW,
    SE,
}

impl Quadrant {
    pub fn new<F: GeoNum>(dx: F, dy: F) -> Option<Quadrant> {
        if dx.is_zero() && dy.is_zero() {
            return None;
        }
        if dx >= F::zero() {
            if dy >= F::zero() {
                Some(Quadrant::NE)
            } else {
                Some(Quadrant::SE)
            }
        } else {
            if dy >= F::zero() {
                Some(Quadrant::NW)
            } else {
                Some(Quadrant::SW)
            }
        }
    }
}
