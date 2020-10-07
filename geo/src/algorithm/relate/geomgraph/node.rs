// JTS: import org.locationtech.jts.geom.Coordinate;
// JTS: import org.locationtech.jts.geom.IntersectionMatrix;
// JTS: import org.locationtech.jts.geom.Location;
use super::{CoordPos, Dimensions, EdgeEnd, EdgeEndBundleStar, IntersectionMatrix, Label};
use crate::{Coordinate, GeoFloat};

// JTS: /**
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class Node
// JTS:   extends GraphComponent
// JTS: {
#[derive(Debug, Clone)]
pub(crate) struct CoordNode<F>
where
    F: GeoFloat,
{
    coordinate: Coordinate<F>,
    label: Label,
}

impl<F: GeoFloat> CoordNode<F> {
    pub(crate) fn label(&self) -> &Label {
        &self.label
    }

    pub(crate) fn label_mut(&mut self) -> &mut Label {
        &mut self.label
    }

    pub(crate) fn is_isolated(&self) -> bool {
        self.label.geometry_count() == 1
    }
}

impl<F> CoordNode<F>
where
    F: GeoFloat,
{
    // JTS:   protected Coordinate coord; // only non-null if this node is precise
    // JTS:   public void add(EdgeEnd e)
    // JTS:   {
    // JTS:     // Assert: start pt of e is equal to node point
    // JTS:     edges.insert(e);
    // JTS:     e.setNode(this);
    // JTS:   }

    // JTS:   protected EdgeEndStar edges;
    // JTS:
    // JTS:   public Node(Coordinate coord, EdgeEndStar edges)
    // JTS:   {
    // JTS:     this.coord = coord;
    // JTS:     this.edges = edges;
    // JTS:     label = new Label(0, Location.NONE);
    // JTS:   }
    pub fn new(coordinate: Coordinate<F>) -> CoordNode<F> {
        CoordNode {
            coordinate,
            label: Label::empty_line_or_point(),
        }
    }

    // JTS:   public Coordinate getCoordinate() { return coord; }
    pub fn coordinate(&self) -> &Coordinate<F> {
        &self.coordinate
    }

    // JTS:   public EdgeEndStar getEdges() { return edges; }

    // JTS:
    // JTS:   /**
    // JTS:    * Tests whether any incident edge is flagged as
    // JTS:    * being in the result.
    // JTS:    * This test can be used to determine if the node is in the result,
    // JTS:    * since if any incident edge is in the result, the node must be in the result as well.
    // JTS:    *
    // JTS:    * @return <code>true</code> if any incident edge in the in the result
    // JTS:    */
    // JTS:   public boolean isIncidentEdgeInResult()
    // JTS:   {
    // JTS:     for (Iterator it = getEdges().getEdges().iterator(); it.hasNext(); ) {
    // JTS:       DirectedEdge de = (DirectedEdge) it.next();
    // JTS:       if (de.getEdge().isInResult())
    // JTS:         return true;
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   public boolean isIsolated()
    // JTS:   {
    // JTS:     return (label.getGeometryCount() == 1);
    // JTS:   }
    // JTS:   /**
    // JTS:    * Basic nodes do not compute IMs
    // JTS:    */
    // JTS:   protected void computeIM(IntersectionMatrix im) {}
    // JTS:   /**
    // JTS:    * Add the edge to the list of edges at this node
    // JTS:    */
    // JTS:   public void mergeLabel(Node n)
    // JTS:   {
    // JTS:     mergeLabel(n.label);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * To merge labels for two nodes,
    // JTS:    * the merged location for each LabelElement is computed.
    // JTS:    * The location for the corresponding node LabelElement is set to the result,
    // JTS:    * as long as the location is non-null.
    // JTS:    */
    // JTS:
    // JTS:   public void mergeLabel(Label label2)
    // JTS:   {
    // JTS:     for (int i = 0; i < 2; i++) {
    // JTS:       int loc = computeMergedLocation(label2, i);
    // JTS:       int thisLoc = label.getLocation(i);
    // JTS:       if (thisLoc == Location.NONE) label.setLocation(i, loc);
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   public void setLabel(int argIndex, int onLocation)
    // JTS:   {
    // JTS:     if (label == null) {
    // JTS:       label = new Label(argIndex, onLocation);
    // JTS:     }
    // JTS:     else
    // JTS:       label.setLocation(argIndex, onLocation);
    // JTS:   }
    pub fn set_label_on_position(&mut self, geom_index: usize, position: CoordPos) {
        self.label.set_on_position(geom_index, position)
    }

    // JTS:   /**
    // JTS:    * Updates the label of a node to BOUNDARY,
    // JTS:    * obeying the mod-2 boundaryDetermination rule.
    // JTS:    */
    // JTS:   public void setLabelBoundary(int argIndex)
    // JTS:   {
    // JTS:     if (label == null) return;
    // JTS:
    // JTS:     // determine the current location for the point (if any)
    // JTS:     int loc = Location.NONE;
    // JTS:     if (label != null)
    // JTS:       loc = label.getLocation(argIndex);
    // JTS:     // flip the loc
    // JTS:     int newLoc;
    // JTS:     switch (loc) {
    // JTS:     case Location.BOUNDARY: newLoc = Location.INTERIOR; break;
    // JTS:     case Location.INTERIOR: newLoc = Location.BOUNDARY; break;
    // JTS:     default: newLoc = Location.BOUNDARY;  break;
    // JTS:     }
    // JTS:     label.setLocation(argIndex, newLoc);
    // JTS:   }
    /// Updates the label of a node to BOUNDARY, obeying the mod-2 rule.
    pub fn set_label_boundary(&mut self, geom_index: usize) {
        let new_position = match self.label.on_position(geom_index) {
            Some(CoordPos::OnBoundary) => CoordPos::Inside,
            Some(CoordPos::Inside) => CoordPos::OnBoundary,
            None | Some(CoordPos::Outside) => CoordPos::OnBoundary,
        };
        self.label.set_on_position(geom_index, new_position);
    }

    // JTS:
    // JTS:   /**
    // JTS:    * The location for a given eltIndex for a node will be one
    // JTS:    * of { null, INTERIOR, BOUNDARY }.
    // JTS:    * A node may be on both the boundary and the interior of a geometry;
    // JTS:    * in this case, the rule is that the node is considered to be in the boundary.
    // JTS:    * The merged location is the maximum of the two input values.
    // JTS:    */
    // JTS:   int computeMergedLocation(Label label2, int eltIndex)
    // JTS:   {
    // JTS:     int loc = Location.NONE;
    // JTS:     loc = label.getLocation(eltIndex);
    // JTS:     if (! label2.isNull(eltIndex)) {
    // JTS:         int nLoc = label2.getLocation(eltIndex);
    // JTS:         if (loc != Location.BOUNDARY) loc = nLoc;
    // JTS:     }
    // JTS:     return loc;
    // JTS:   }
    // JTS:
    // JTS:   public void print(PrintStream out)
    // JTS:   {
    // JTS:     out.println("node " + coord + " lbl: " + label);
    // JTS:   }
    // JTS: }
}

impl<F: GeoFloat> CoordNode<F> {
    // from JTS#GraphComponent - seems like only node uses this impl, so implementing it directly
    // onto node rather than the GraphComponent trait
    // JTS:   /**
    // JTS:    * Update the IM with the contribution for this component.
    // JTS:    * A component only contributes if it has a labelling for both parent geometries
    // JTS:    */
    // JTS:   public void updateIM(IntersectionMatrix im)
    // JTS:   {
    // JTS:     Assert.isTrue(label.getGeometryCount() >= 2, "found partial label");
    // JTS:     computeIM(im);
    // JTS:   }
    pub fn update_intersection_matrix(&self, intersection_matrix: &mut IntersectionMatrix) {
        assert!(self.label.geometry_count() >= 2, "found partial label");
        intersection_matrix.set_at_least_if_valid(
            self.label.on_position(0),
            self.label.on_position(1),
            Dimensions::ZeroDimensional,
        );
        debug!(
            "updated intersection_matrix: {:?} from node: {:?}",
            intersection_matrix, self
        );
    }

    // from JTS#RelateNode, which we've squashed into the base Node class to avoid wrestling OO hierarchies into rust.
    // JTS:   /**
    // JTS:    * Update the IM with the contribution for this component.
    // JTS:    * A component only contributes if it has a labelling for both parent geometries
    // JTS:    */
    // JTS:   protected void computeIM(IntersectionMatrix im)
    // JTS:   {
    // JTS:     im.setAtLeastIfValid(label.getLocation(0), label.getLocation(1), 0);
    // JTS:   }
    // JTS:   /**
    // JTS:    * Update the IM with the contribution for the EdgeEnds incident on this node.
    // JTS:    */
    // JTS:   void updateIMFromEdges(IntersectionMatrix im)
    // JTS:   {
    // JTS:     ((EdgeEndBundleStar) edges).updateIM(im);
    // JTS:   }
    // JTS: }
}
