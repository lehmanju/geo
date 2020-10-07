use crate::{Coordinate, GeoFloat};

// JTS: /**
// JTS:  * Represents a point on an
// JTS:  * edge which intersects with another edge.
// JTS:  * <p>
// JTS:  * The intersection may either be a single point, or a line segment
// JTS:  * (in which case this point is the start of the line segment)
// JTS:  * The intersection point must be precise.
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class EdgeIntersection
// JTS:     implements Comparable
// JTS: {

/// Represents a point on an edge which intersects with another edge.
///
/// The intersection may either be a single point, or a line segment (in which case this point is
/// the start of the line segment) The intersection point must be precise.
///
/// This is based on [JTS's EdgeIntersection as of 1.18.1](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/geomgraph/EdgeIntersection.java)
#[derive(Debug)]
pub(crate) struct EdgeIntersection<F: GeoFloat> {
    coord: Coordinate<F>,
    segment_index: usize,
    dist: F,
}

// JTS:   public Coordinate coord;   // the point of intersection
// JTS:   public int segmentIndex;   // the index of the containing line segment in the parent edge
// JTS:   public double dist;        // the edge distance of this point along the containing line segment
// JTS:
impl<F: GeoFloat> EdgeIntersection<F> {
    // JTS:   public EdgeIntersection(Coordinate coord, int segmentIndex, double dist) {
    pub fn new(coord: Coordinate<F>, segment_index: usize, dist: F) -> EdgeIntersection<F> {
        // JTS:     this.coord = new Coordinate(coord);
        // JTS:     this.segmentIndex = segmentIndex;
        // JTS:     this.dist = dist;
        // JTS:   }
        EdgeIntersection {
            coord,
            segment_index,
            dist,
        }
    }

    // JTS:   public Coordinate getCoordinate() { return coord; }
    pub fn coordinate(&self) -> Coordinate<F> {
        self.coord
    }

    // JTS:   public int getSegmentIndex() { return segmentIndex; }
    pub fn segment_index(&self) -> usize {
        self.segment_index
    }

    // JTS:   public double getDistance() { return dist; }
    pub fn distance(&self) -> F {
        self.dist
    }
}

impl<F: GeoFloat> std::cmp::PartialEq for EdgeIntersection<F> {
    fn eq(&self, other: &EdgeIntersection<F>) -> bool {
        self.segment_index == other.segment_index && self.dist == other.dist
    }
}

impl<F: GeoFloat> std::cmp::Eq for EdgeIntersection<F> {}

impl<F: GeoFloat> std::cmp::PartialOrd for EdgeIntersection<F> {
    fn partial_cmp(&self, other: &EdgeIntersection<F>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: GeoFloat> std::cmp::Ord for EdgeIntersection<F> {
    fn cmp(&self, other: &EdgeIntersection<F>) -> std::cmp::Ordering {
        // JTS:   public int compareTo(Object obj)
        // JTS:   {
        // JTS:     EdgeIntersection other = (EdgeIntersection) obj;
        // JTS:     return compare(other.segmentIndex, other.dist);
        // JTS:   }
        // JTS:   /**
        // JTS:    * @return -1 this EdgeIntersection is located before the argument location
        // JTS:    * @return 0 this EdgeIntersection is at the argument location
        // JTS:    * @return 1 this EdgeIntersection is located after the argument location
        // JTS:    */
        // JTS:   public int compare(int segmentIndex, double dist)
        // JTS:   {
        // JTS:     if (this.segmentIndex < segmentIndex) return -1;
        // JTS:     if (this.segmentIndex > segmentIndex) return 1;
        // JTS:     if (this.dist < dist) return -1;
        // JTS:     if (this.dist > dist) return 1;
        // JTS:     return 0;
        // JTS:   }
        if self.segment_index < other.segment_index {
            return std::cmp::Ordering::Less;
        }
        if self.segment_index > other.segment_index {
            return std::cmp::Ordering::Greater;
        }
        if self.dist < other.dist {
            return std::cmp::Ordering::Less;
        }
        if self.dist > other.dist {
            return std::cmp::Ordering::Greater;
        }

        // BTreeMap requires nodes to be fully `Ord`, but we're comparing floats, so we require
        // non-NaN for valid results.
        debug_assert!(!self.dist.is_nan() && !other.dist.is_nan());

        std::cmp::Ordering::Equal
    }
}

impl<F: GeoFloat> EdgeIntersection<F> {
    // JTS:
    // JTS:   public boolean isEndPoint(int maxSegmentIndex)
    // JTS:   {
    // JTS:     if (segmentIndex == 0 && dist == 0.0) return true;
    // JTS:     if (segmentIndex == maxSegmentIndex) return true;
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   public void print(PrintStream out)
    // JTS:   {
    // JTS:     out.print(coord);
    // JTS:     out.print(" seg # = " + segmentIndex);
    // JTS:     out.println(" dist = " + dist);
    // JTS:   }
    // JTS:   public String toString()
    // JTS:   {
    // JTS:     return coord + " seg # = " + segmentIndex + " dist = " + dist;
    // JTS:   }
    // JTS: }
}
