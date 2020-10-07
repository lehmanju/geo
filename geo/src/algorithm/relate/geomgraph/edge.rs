use super::{Dimensions, Direction, EdgeIntersection, IntersectionMatrix, Label};
use super::{LineIntersection, LineIntersector, RobustLineIntersector};
use crate::{Coordinate, GeoFloat, Line};

use std::collections::BTreeSet;

/// An `Edge` represents a one dimensional line in a geometry.
///
/// This is based on [JTS's `Edge` as of 1.18.1](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/geomgraph/Edge.java)
#[derive(Debug)]
pub(crate) struct Edge<F: GeoFloat> {
    /// `coordinates` of the line geometry
    coords: Vec<Coordinate<F>>,

    /// an edge is "isolated" if no other edge touches it
    is_isolated: bool,

    /// other edges that this edge intersects with
    edge_intersections: BTreeSet<EdgeIntersection<F>>,

    /// where the line's topological classification to the two geometries is recorded
    label: Label,
}

// JTS: /**
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class Edge
// JTS:   extends GraphComponent
// JTS: {
impl<F: GeoFloat> Edge<F> {
    // JTS:   private Depth depth = new Depth();
    // JTS:   private int depthDelta = 0;   // the change in area depth from the R to L side of this edge
    // JTS:
    // JTS:   public Edge(Coordinate[] pts, Label label)
    // JTS:   {
    // JTS:     this.pts = pts;
    // JTS:     this.label = label;
    // JTS:   }
    /// Create a new Edge.
    ///
    /// - `coords` a *non-empty* Vec of Coordinates
    /// - `label` an appropriately dimensioned topology label for the Edge. See [`TopologyPosition`]
    ///    for details
    pub(crate) fn new(coords: Vec<Coordinate<F>>, label: Label) -> Edge<F> {
        assert!(!coords.is_empty(), "Can't add empty edge");
        Edge {
            coords,
            label,
            is_isolated: true,
            edge_intersections: BTreeSet::new(),
        }
    }

    pub(crate) fn label(&self) -> &Label {
        &self.label
    }

    pub(crate) fn label_mut(&mut self) -> &mut Label {
        &mut self.label
    }

    // JTS:   Coordinate[] pts;
    pub fn coords(&self) -> &[Coordinate<F>] {
        &self.coords
    }

    // JTS:   private Envelope env;
    // JTS:   EdgeIntersectionList eiList = new EdgeIntersectionList(this);
    // JTS:   private String name;
    // JTS:   private MonotoneChainEdge mce;

    // JTS:   private boolean isIsolated = true;
    pub fn is_isolated(&self) -> bool {
        self.is_isolated
    }
    pub fn mark_as_unisolated(&mut self) {
        self.is_isolated = false;
    }

    // JTS:   public Edge(Coordinate[] pts)
    // JTS:   {
    // JTS:     this(pts, null);
    // JTS:   }
    // JTS:
    // JTS:   public int getNumPoints() { return pts.length; }
    // JTS:   public void setName(String name) { this.name = name; }
    // JTS:   public Coordinate[] getCoordinates()  {    return pts;  }
    // JTS:   public Coordinate getCoordinate(int i)
    // JTS:   {
    // JTS:     return pts[i];
    // JTS:   }
    // JTS:   public Coordinate getCoordinate()
    // JTS:   {
    // JTS:     if (pts.length > 0) return pts[0];
    // JTS:     return null;
    // JTS:   }
    // JTS:   public Envelope getEnvelope()
    // JTS:   {
    // JTS:     // compute envelope lazily
    // JTS:     if (env == null) {
    // JTS:       env = new Envelope();
    // JTS:       for (int i = 0; i < pts.length; i++) {
    // JTS:         env.expandToInclude(pts[i]);
    // JTS:       }
    // JTS:     }
    // JTS:     return env;
    // JTS:   }
    // JTS:
    // JTS:   public Depth getDepth() { return depth; }
    // JTS:
    // JTS:   /**
    // JTS:    * The depthDelta is the change in depth as an edge is crossed from R to L
    // JTS:    * @return the change in depth as the edge is crossed from R to L
    // JTS:    */
    // JTS:   public int getDepthDelta()  { return depthDelta;  }
    // JTS:   public void setDepthDelta(int depthDelta)  { this.depthDelta = depthDelta;  }
    // JTS:
    // JTS:   public int getMaximumSegmentIndex()
    // JTS:   {
    // JTS:     return pts.length - 1;
    // JTS:   }

    // JTS:   public EdgeIntersectionList getEdgeIntersectionList() { return eiList; }
    pub fn edge_intersections(&self) -> &BTreeSet<EdgeIntersection<F>> {
        &self.edge_intersections
    }

    pub fn edge_intersections_mut(&mut self) -> &mut BTreeSet<EdgeIntersection<F>> {
        &mut self.edge_intersections
    }

    pub fn add_edge_intersection_list_endpoints(&mut self) {
        let max_segment_index = self.coords().len() - 1;
        let first_coord = self.coords()[0];
        let max_coord = self.coords()[max_segment_index];
        self.edge_intersections_mut()
            .insert(EdgeIntersection::new(first_coord, 0, F::zero()));
        self.edge_intersections_mut().insert(EdgeIntersection::new(
            max_coord,
            max_segment_index,
            F::zero(),
        ));
    }

    // JTS:
    // JTS:   public MonotoneChainEdge getMonotoneChainEdge()
    // JTS:   {
    // JTS:     if (mce == null) mce = new MonotoneChainEdge(this);
    // JTS:     return mce;
    // JTS:   }

    // JTS:   public boolean isClosed()
    // JTS:   {
    // JTS:     return pts[0].equals(pts[pts.length - 1]);
    // JTS:   }
    pub fn is_closed(&self) -> bool {
        self.coords().first() == self.coords().last()
    }
    // JTS:   /**
    // JTS:    * An Edge is collapsed if it is an Area edge and it consists of
    // JTS:    * two segments which are equal and opposite (eg a zero-width V).
    // JTS:    */
    // JTS:   public boolean isCollapsed()
    // JTS:   {
    // JTS:     if (! label.isArea()) return false;
    // JTS:     if (pts.length != 3) return false;
    // JTS:     if (pts[0].equals(pts[2]) ) return true;
    // JTS:     return false;
    // JTS:   }
    // JTS:   public Edge getCollapsedEdge()
    // JTS:   {
    // JTS:     Coordinate newPts[] = new Coordinate[2];
    // JTS:     newPts[0] = pts[0];
    // JTS:     newPts[1] = pts[1];
    // JTS:     Edge newe = new Edge(newPts, Label.toLineLabel(label));
    // JTS:     return newe;
    // JTS:   }
    // JTS:
    // JTS:   public void setIsolated(boolean isIsolated)
    // JTS:   {
    // JTS:     this.isIsolated = isIsolated;
    // JTS:   }
    // JTS:   public boolean isIsolated()
    // JTS:   {
    // JTS:     return isIsolated;
    // JTS:   }

    // JTS:   /**
    // JTS:    * Adds EdgeIntersections for one or both
    // JTS:    * intersections found for a segment of an edge to the edge intersection list.
    // JTS:    */
    // JTS:   public void addIntersections(LineIntersector li, int segmentIndex, int geomIndex)
    // JTS:   {
    // JTS:     for (int i = 0; i < li.getIntersectionNum(); i++) {
    // JTS:       addIntersection(li, segmentIndex, geomIndex, i);
    // JTS:     }
    // JTS:   }
    /// Adds EdgeIntersections for one or both intersections found for a segment of an edge to the
    /// edge intersection list.
    pub fn add_intersections(
        &mut self,
        intersection: LineIntersection<F>,
        line: Line<F>,
        segment_index: usize,
    ) {
        match intersection {
            LineIntersection::SinglePoint { intersection, .. } => {
                self.add_intersection(intersection, line, segment_index);
            }
            LineIntersection::Collinear { intersection } => {
                self.add_intersection(intersection.start, line, segment_index);
                self.add_intersection(intersection.end, line, segment_index);
            }
        }
    }

    // JTS:   /**
    // JTS:    * Add an EdgeIntersection for intersection intIndex.
    // JTS:    * An intersection that falls exactly on a vertex of the edge is normalized
    // JTS:    * to use the higher of the two possible segmentIndexes
    // JTS:    */
    // JTS:   public void addIntersection(LineIntersector li, int segmentIndex, int geomIndex, int intIndex)
    // JTS:   {
    /// Add an EdgeIntersection for intersection intIndex.
    /// An intersection that falls exactly on a vertex of the edge is normalized to use the higher
    /// of the two possible segmentIndexes
    pub fn add_intersection(
        &mut self,
        intersection_coord: Coordinate<F>,
        line: Line<F>,
        segment_index: usize,
    ) {
        // JTS:       Coordinate intPt = new Coordinate(li.getIntersection(intIndex));
        // JTS:       int normalizedSegmentIndex = segmentIndex;
        // JTS:       double dist = li.getEdgeDistance(geomIndex, intIndex);
        let mut normalized_segment_index = segment_index;
        let mut distance = RobustLineIntersector::compute_edge_distance(intersection_coord, line);

        // JTS: //Debug.println("edge intpt: " + intPt + " dist: " + dist);
        // JTS:       // normalize the intersection point location
        // JTS:       int nextSegIndex = normalizedSegmentIndex + 1;
        let next_segment_index = normalized_segment_index + 1;

        // JTS:       if (nextSegIndex < pts.length) {
        if next_segment_index < self.coords.len() {
            // JTS:         Coordinate nextPt = pts[nextSegIndex];
            let next_coord = self.coords[next_segment_index];
            // JTS: //Debug.println("next pt: " + nextPt);
            // JTS:
            // JTS:         // Normalize segment index if intPt falls on vertex
            // JTS:         // The check for point equality is 2D only - Z values are ignored
            // JTS:         if (intPt.equals2D(nextPt)) {
            // JTS: //Debug.println("normalized distance");
            // JTS:             normalizedSegmentIndex = nextSegIndex;
            // JTS:             dist = 0.0;
            // JTS:         }
            if intersection_coord == next_coord {
                normalized_segment_index = next_segment_index;
                distance = F::zero();
            }
            // JTS:       }
        }
        // JTS:       /**
        // JTS:       * Add the intersection point to edge intersection list.
        // JTS:       */
        // JTS:       EdgeIntersection ei = eiList.add(intPt, normalizedSegmentIndex, dist);
        // JTS: //ei.print(System.out);
        self.edge_intersections.insert(EdgeIntersection::new(
            intersection_coord,
            normalized_segment_index,
            distance,
        ));
    }
    // JTS:
    // JTS:   /**
    // JTS:    * Update the IM with the contribution for this component.
    // JTS:    * A component only contributes if it has a labelling for both parent geometries
    // JTS:    */
    // JTS:   public void computeIM(IntersectionMatrix im)
    // JTS:   {
    // JTS:     updateIM(label, im);
    // JTS:   }
    // JTS:   /**
    // JTS:    * Updates an IM from the label for an edge.
    // JTS:    * Handles edges from both L and A geometries.
    // JTS:    */
    // JTS:   public static void updateIM(Label label, IntersectionMatrix im)
    // JTS:   {
    // JTS:     im.setAtLeastIfValid(label.getLocation(0, Position.ON), label.getLocation(1, Position.ON), 1);
    // JTS:     if (label.isArea()) {
    // JTS:       im.setAtLeastIfValid(label.getLocation(0, Position.LEFT),  label.getLocation(1, Position.LEFT),   2);
    // JTS:       im.setAtLeastIfValid(label.getLocation(0, Position.RIGHT), label.getLocation(1, Position.RIGHT),  2);
    // JTS:     }
    // JTS:   }
    pub fn update_intersection_matrix(label: &Label, intersection_matrix: &mut IntersectionMatrix) {
        intersection_matrix.set_at_least_if_valid(
            label.position(0, Direction::On),
            label.position(1, Direction::On),
            Dimensions::OneDimensional,
        );

        if label.is_area() {
            intersection_matrix.set_at_least_if_valid(
                label.position(0, Direction::Left),
                label.position(1, Direction::Left),
                Dimensions::TwoDimensional,
            );
            intersection_matrix.set_at_least_if_valid(
                label.position(0, Direction::Right),
                label.position(1, Direction::Right),
                Dimensions::TwoDimensional,
            );
        }
    }
}

// JTS:   /**
// JTS:    * @return true if the coordinate sequences of the Edges are identical
// JTS:    */
// JTS:   public boolean isPointwiseEqual(Edge e)
// JTS:   {
// JTS:     if (pts.length != e.pts.length) return false;
// JTS:
// JTS:     for (int i = 0; i < pts.length; i++) {
// JTS:       if (! pts[i].equals2D(e.pts[i])) {
// JTS:          return false;
// JTS:       }
// JTS:     }
// JTS:     return true;
// JTS:   }
// JTS:
// JTS:   public String toString()
// JTS:   {
// JTS:     StringBuilder builder = new StringBuilder();
// JTS:     builder.append("edge " + name + ": ");
// JTS:     builder.append("LINESTRING (");
// JTS:     for (int i = 0; i < pts.length; i++) {
// JTS:       if (i > 0) builder.append(",");
// JTS:       builder.append(pts[i].x + " " + pts[i].y);
// JTS:     }
// JTS:     builder.append(")  " + label + " " + depthDelta);
// JTS:     return builder.toString();
// JTS:   }
// JTS:   public void print(PrintStream out)
// JTS:   {
// JTS:     out.print("edge " + name + ": ");
// JTS:     out.print("LINESTRING (");
// JTS:     for (int i = 0; i < pts.length; i++) {
// JTS:       if (i > 0) out.print(",");
// JTS:       out.print(pts[i].x + " " + pts[i].y);
// JTS:     }
// JTS:     out.print(")  " + label + " " + depthDelta);
// JTS:   }
// JTS:   public void printReverse(PrintStream out)
// JTS:   {
// JTS:     out.print("edge " + name + ": ");
// JTS:     for (int i = pts.length - 1; i >= 0; i--) {
// JTS:       out.print(pts[i] + " ");
// JTS:     }
// JTS:     out.println("");
// JTS:   }
// JTS:
// JTS: }
// JTS:
