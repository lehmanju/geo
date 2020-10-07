use crate::GeoNum;

// JTS: /**
// JTS:  * Utility functions for working with quadrants, which are numbered as follows:
// JTS:  * <pre>
// JTS:  * 1 | 0
// JTS:  * --+--
// JTS:  * 2 | 3
// JTS:  * </pre>
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class Quadrant
// JTS: {
// JTS:     public static final int NE = 0;
// JTS:     public static final int NW = 1;
// JTS:     public static final int SW = 2;
// JTS:     public static final int SE = 3;
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
    // JTS:
    // JTS:   /**
    // JTS:    * Returns the quadrant of a directed line segment (specified as x and y
    // JTS:    * displacements, which cannot both be 0).
    // JTS:    *
    // JTS:    * @throws IllegalArgumentException if the displacements are both 0
    // JTS:    */
    // JTS:   public static int quadrant(double dx, double dy)
    // JTS:   {
    pub fn new<F: GeoNum>(dx: F, dy: F) -> Option<Quadrant> {
        // JTS:     if (dx == 0.0 && dy == 0.0)
        // JTS:       throw new IllegalArgumentException("Cannot compute the quadrant for point ( "+ dx + ", " + dy + " )" );
        if dx.is_zero() && dy.is_zero() {
            return None;
        }
        // JTS:     if (dx >= 0.0) {
        // JTS:       if (dy >= 0.0)
        // JTS:         return NE;
        // JTS:       else
        // JTS:         return SE;
        // JTS:     }
        if dx >= F::zero() {
            if dy >= F::zero() {
                Some(Quadrant::NE)
            } else {
                Some(Quadrant::SE)
            }
        // JTS:     else {
        } else {
            // JTS:         if (dy >= 0.0)
            // JTS:             return NW;
            // JTS:         else
            // JTS:             return SW;
            // JTS:     }
            // JTS:   }
            if dy >= F::zero() {
                Some(Quadrant::NW)
            } else {
                Some(Quadrant::SW)
            }
        }
    }
    // JTS:
    // JTS:   /**
    // JTS:    * Returns the quadrant of a directed line segment from p0 to p1.
    // JTS:    *
    // JTS:    * @throws IllegalArgumentException if the points are equal
    // JTS:    */
    // JTS:   public static int quadrant(Coordinate p0, Coordinate p1)
    // JTS:   {
    // JTS:     if (p1.x == p0.x && p1.y == p0.y)
    // JTS:       throw new IllegalArgumentException("Cannot compute the quadrant for two identical points " + p0);
    // JTS:
    // JTS:     if (p1.x >= p0.x) {
    // JTS:       if (p1.y >= p0.y)
    // JTS:         return NE;
    // JTS:       else
    // JTS:         return SE;
    // JTS:     }
    // JTS:     else {
    // JTS:         if (p1.y >= p0.y)
    // JTS:             return NW;
    // JTS:         else
    // JTS:             return SW;
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Returns true if the quadrants are 1 and 3, or 2 and 4
    // JTS:    */
    // JTS:   public static boolean isOpposite(int quad1, int quad2)
    // JTS:   {
    // JTS:     if (quad1 == quad2) return false;
    // JTS:     int diff = (quad1 - quad2 + 4) % 4;
    // JTS:     // if quadrants are not adjacent, they are opposite
    // JTS:     if (diff == 2) return true;
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Returns the right-hand quadrant of the halfplane defined by the two quadrants,
    // JTS:    * or -1 if the quadrants are opposite, or the quadrant if they are identical.
    // JTS:    */
    // JTS:   public static int commonHalfPlane(int quad1, int quad2)
    // JTS:   {
    // JTS:     // if quadrants are the same they do not determine a unique common halfplane.
    // JTS:     // Simply return one of the two possibilities
    // JTS:     if (quad1 == quad2) return quad1;
    // JTS:     int diff = (quad1 - quad2 + 4) % 4;
    // JTS:     // if quadrants are not adjacent, they do not share a common halfplane
    // JTS:     if (diff == 2) return -1;
    // JTS:     //
    // JTS:     int min = (quad1 < quad2) ? quad1 : quad2;
    // JTS:     int max = (quad1 > quad2) ? quad1 : quad2;
    // JTS:     // for this one case, the righthand plane is NOT the minimum index;
    // JTS:     if (min == 0 && max == 3) return 3;
    // JTS:     // in general, the halfplane index is the minimum of the two adjacent quadrants
    // JTS:     return min;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Returns whether the given quadrant lies within the given halfplane (specified
    // JTS:    * by its right-hand quadrant).
    // JTS:    */
    // JTS:   public static boolean isInHalfPlane(int quad, int halfPlane)
    // JTS:   {
    // JTS:     if (halfPlane == SE) {
    // JTS:       return quad == SE || quad == SW;
    // JTS:     }
    // JTS:     return quad == halfPlane || quad == halfPlane + 1;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Returns true if the given quadrant is 0 or 1.
    // JTS:    */
    // JTS:   public static boolean isNorthern(int quad)
    // JTS:   {
    // JTS:     return quad == NE || quad == NW;
    // JTS:   }
    // JTS: }
}
