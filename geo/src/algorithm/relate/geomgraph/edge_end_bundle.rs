use super::{CoordPos, Direction, Edge, EdgeEnd, GeometryGraph, IntersectionMatrix, Label};
use crate::{Coordinate, GeoFloat};

// JTS: import org.locationtech.jts.algorithm.BoundaryNodeRule;
// JTS: import org.locationtech.jts.geom.IntersectionMatrix;
// JTS: import org.locationtech.jts.geom.Location;
// JTS: import org.locationtech.jts.geomgraph.Edge;
// JTS: import org.locationtech.jts.geomgraph.EdgeEnd;
// JTS: import org.locationtech.jts.geomgraph.GeometryGraph;
// JTS: import org.locationtech.jts.geomgraph.Label;
// JTS: import org.locationtech.jts.geomgraph.Position;
// JTS:
// JTS: /**
// JTS:  * A collection of {@link EdgeEnd}s which obey the following invariant:
// JTS:  * They originate at the same node and have the same direction.
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class EdgeEndBundle
// JTS:   extends EdgeEnd
/// A collection of [`EdgeEnds`](EdgeEnd) which obey the following invariant:
/// They originate at the same node and have the same direction.
///
/// This is based on [JTS's `EdgeEndBundle` as of 1.18.1](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/operation/relate/EdgeEndBundle.java)
#[derive(Clone, Debug)]
pub(crate) struct EdgeEndBundle<F>
where
    F: GeoFloat,
{
    coordinate: Coordinate<F>,
    edge_ends: Vec<EdgeEnd<F>>,
}

impl<F> EdgeEndBundle<F>
where
    F: GeoFloat,
{
    // JTS: {
    // JTS: //  private BoundaryNodeRule boundaryNodeRule;
    // JTS:   private List edgeEnds = new ArrayList();
    // JTS:
    // JTS:   public EdgeEndBundle(BoundaryNodeRule boundaryNodeRule, EdgeEnd e)
    // JTS:   {
    // JTS:     super(e.getEdge(), e.getCoordinate(), e.getDirectedCoordinate(), new Label(e.getLabel()));
    // JTS:     insert(e);
    // JTS:     /*
    // JTS:     if (boundaryNodeRule != null)
    // JTS:       this.boundaryNodeRule = boundaryNodeRule;
    // JTS:     else
    // JTS:       boundaryNodeRule = BoundaryNodeRule.OGC_SFS_BOUNDARY_RULE;
    // JTS:     */
    // JTS:   }
    // JTS:
    // JTS:   public EdgeEndBundle(EdgeEnd e)
    // JTS:   {
    // JTS:     this(null, e);
    // JTS:   }
    pub(crate) fn new(coordinate: Coordinate<F>) -> Self {
        Self {
            coordinate,
            edge_ends: vec![],
        }
    }

    // JTS:   public Label getLabel() { return label; }

    // JTS:   public Iterator iterator() { return edgeEnds.iterator(); }
    // JTS:   public List getEdgeEnds() { return edgeEnds; }
    fn edge_ends_iter(&self) -> impl Iterator<Item = &EdgeEnd<F>> {
        self.edge_ends.iter()
    }

    fn edge_ends_iter_mut(&mut self) -> impl Iterator<Item = &mut EdgeEnd<F>> {
        self.edge_ends.iter_mut()
    }

    // JTS:
    // JTS:   public void insert(EdgeEnd e)
    // JTS:   {
    // JTS:     // Assert: start point is the same
    // JTS:     // Assert: direction is the same
    // JTS:     edgeEnds.add(e);
    // JTS:   }
    pub(crate) fn insert(&mut self, edge_end: EdgeEnd<F>) {
        self.edge_ends.push(edge_end);
    }

    // JTS:   /**
    // JTS:    * This computes the overall edge label for the set of
    // JTS:    * edges in this EdgeStubBundle.  It essentially merges
    // JTS:    * the ON and side labels for each edge.  These labels must be compatible
    // JTS:    */
    // JTS:   public void computeLabel(BoundaryNodeRule boundaryNodeRule)
    // JTS:   {
    pub(crate) fn into_labeled(mut self) -> LabeledEdgeEndBundle<F> {
        // JTS:     // create the label.  If any of the edges belong to areas,
        // JTS:     // the label must be an area label
        // JTS:     boolean isArea = false;
        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        // JTS:       EdgeEnd e = (EdgeEnd) it.next();
        // JTS:       if (e.getLabel().isArea()) isArea = true;
        // JTS:     }
        let is_area = self
            .edge_ends_iter()
            .any(|edge_end| edge_end.label().is_area());

        // JTS:     if (isArea)
        // JTS:       label = new Label(Location.NONE, Location.NONE, Location.NONE);
        // JTS:     else
        // JTS:       label = new Label(Location.NONE);
        let mut label = if is_area {
            Label::empty_area()
        } else {
            Label::empty_line_or_point()
        };

        // JTS:     // compute the On label, and the side labels if present
        // JTS:     for (int i = 0; i < 2; i++) {
        // JTS:       computeLabelOn(i, boundaryNodeRule);
        // JTS:       if (isArea)
        // JTS:         computeLabelSides(i);
        // JTS:     }
        // JTS:   }
        for i in 0..2 {
            self.compute_label_on(&mut label, i);
            if is_area {
                self.compute_label_side(&mut label, i, Direction::Left);
                self.compute_label_side(&mut label, i, Direction::Right);
            }
        }

        LabeledEdgeEndBundle {
            label,
            edge_end_bundle: self,
        }
    }

    // JTS:   /**
    // JTS:    * Compute the overall ON location for the list of EdgeStubs.
    // JTS:    * (This is essentially equivalent to computing the self-overlay of a single Geometry)
    // JTS:    * edgeStubs can be either on the boundary (e.g. Polygon edge)
    // JTS:    * OR in the interior (e.g. segment of a LineString)
    // JTS:    * of their parent Geometry.
    // JTS:    * In addition, GeometryCollections use a {@link BoundaryNodeRule} to determine
    // JTS:    * whether a segment is on the boundary or not.
    // JTS:    * Finally, in GeometryCollections it can occur that an edge is both
    // JTS:    * on the boundary and in the interior (e.g. a LineString segment lying on
    // JTS:    * top of a Polygon edge.) In this case the Boundary is given precedence.
    // JTS:    * <br>
    // JTS:    * These observations result in the following rules for computing the ON location:
    // JTS:    * <ul>
    // JTS:    * <li> if there are an odd number of Bdy edges, the attribute is Bdy
    // JTS:    * <li> if there are an even number >= 2 of Bdy edges, the attribute is Int
    // JTS:    * <li> if there are any Int edges, the attribute is Int
    // JTS:    * <li> otherwise, the attribute is NULL.
    // JTS:    * </ul>
    // JTS:    */
    // JTS:   private void computeLabelOn(int geomIndex, BoundaryNodeRule boundaryNodeRule)
    // JTS:   {
    /// Compute the overall ON position for the list of EdgeEnds.
    /// (This is essentially equivalent to computing the self-overlay of a single Geometry)
    ///
    /// EdgeEnds can be either on the boundary (e.g. Polygon edge)
    /// OR in the interior (e.g. segment of a LineString)
    /// of their parent Geometry.
    ///
    /// In addition, GeometryCollections use a boundary node rule to determine whether a segment is
    /// on the boundary or not.
    ///
    /// Finally, in GeometryCollections it can occur that an edge is both
    /// on the boundary and in the interior (e.g. a LineString segment lying on
    /// top of a Polygon edge.) In this case the Boundary is given precedence.
    ///
    /// These observations result in the following rules for computing the ON location:
    /// - if there are an odd number of Bdy edges, the attribute is Bdy
    /// - if there are an even number >= 2 of Bdy edges, the attribute is Int
    /// - if there are any Int edges, the attribute is Int
    /// - otherwise, the attribute is None
    ///
    fn compute_label_on(&mut self, label: &mut Label, geom_index: usize) {
        // JTS:     // compute the ON location value
        // JTS:     int boundaryCount = 0;
        // JTS:     boolean foundInterior = false;
        let mut boundary_count = 0;
        let mut found_interior = false;

        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        // JTS:       EdgeEnd e = (EdgeEnd) it.next();
        // JTS:       int loc = e.getLabel().getLocation(geomIndex);
        // JTS:       if (loc == Location.BOUNDARY) boundaryCount++;
        // JTS:       if (loc == Location.INTERIOR) foundInterior = true;
        // JTS:     }
        for edge_end in self.edge_ends_iter() {
            match edge_end.label().on_position(geom_index) {
                Some(CoordPos::OnBoundary) => {
                    boundary_count += 1;
                }
                Some(CoordPos::Inside) => {
                    found_interior = true;
                }
                None | Some(CoordPos::Outside) => {}
            }
        }

        // JTS:     int loc = Location.NONE;
        // JTS:     if (foundInterior)  loc = Location.INTERIOR;
        // JTS:     if (boundaryCount > 0) {
        // JTS:       loc = GeometryGraph.determineBoundary(boundaryNodeRule, boundaryCount);
        // JTS:     }
        let mut position = None;
        if found_interior {
            position = Some(CoordPos::Inside);
        }

        if boundary_count > 0 {
            position = Some(GeometryGraph::<'_, F>::determine_boundary(boundary_count));
        }

        // JTS:     label.setLocation(geomIndex, loc);
        // JTS:
        // JTS:   }
        if let Some(location) = position {
            label.set_on_position(geom_index, location);
        } else {
            // This is technically a diversion from JTS, but I don't think we'd ever
            // get here, unless `l.on_location` was *already* None, in which cases this is a
            // no-op, so assert that assumption.
            // If this assert is rightfully triggered, we may need to add a method like
            // `l.clear_on_location(geom_index)`
            debug_assert!(
                label.on_position(geom_index).is_none(),
                "diverging from JTS, which would have replaced the existing Location with None"
            );
        }
    }

    // JTS:   /**
    // JTS:    * To compute the summary label for a side, the algorithm is:
    // JTS:    *   FOR all edges
    // JTS:    *     IF any edge's location is INTERIOR for the side, side location = INTERIOR
    // JTS:    *     ELSE IF there is at least one EXTERIOR attribute, side location = EXTERIOR
    // JTS:    *     ELSE  side location = NULL
    // JTS:    *  <br>
    // JTS:    *  Note that it is possible for two sides to have apparently contradictory information
    // JTS:    *  i.e. one edge side may indicate that it is in the interior of a geometry, while
    // JTS:    *  another edge side may indicate the exterior of the same geometry.  This is
    // JTS:    *  not an incompatibility - GeometryCollections may contain two Polygons that touch
    // JTS:    *  along an edge.  This is the reason for Interior-primacy rule above - it
    // JTS:    *  results in the summary label having the Geometry interior on <b>both</b> sides.
    // JTS:    */
    // JTS:   private void computeLabelSide(int geomIndex, int side)
    // JTS:   {
    /// To compute the summary label for a side, the algorithm is:
    ///     FOR all edges
    ///       IF any edge's location is INTERIOR for the side, side location = INTERIOR
    ///       ELSE IF there is at least one EXTERIOR attribute, side location = EXTERIOR
    ///       ELSE  side location = NULL
    /// Note that it is possible for two sides to have apparently contradictory information
    /// i.e. one edge side may indicate that it is in the interior of a geometry, while
    /// another edge side may indicate the exterior of the same geometry.  This is
    /// not an incompatibility - GeometryCollections may contain two Polygons that touch
    /// along an edge.  This is the reason for Interior-primacy rule above - it
    /// results in the summary label having the Geometry interior on _both_ sides.
    fn compute_label_side(&mut self, label: &mut Label, geom_index: usize, side: Direction) {
        let mut position = None;
        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        // JTS:       EdgeEnd e = (EdgeEnd) it.next();
        for edge_end in self.edge_ends_iter_mut() {
            // JTS:       if (e.getLabel().isArea()) {
            if edge_end.label().is_area() {
                // JTS:         int loc = e.getLabel().getLocation(geomIndex, side);
                // JTS:         if (loc == Location.INTERIOR) {
                // JTS:             label.setLocation(geomIndex, side, Location.INTERIOR);
                // JTS:             return;
                // JTS:         }
                // JTS:         else if (loc == Location.EXTERIOR)
                // JTS:               label.setLocation(geomIndex, side, Location.EXTERIOR);
                // JTS:       }
                // JTS:     }
                match edge_end.label_mut().position(geom_index, side) {
                    Some(CoordPos::Inside) => {
                        position = Some(CoordPos::Inside);
                        break;
                    }
                    Some(CoordPos::Outside) => {
                        position = Some(CoordPos::Outside);
                    }
                    None | Some(CoordPos::OnBoundary) => {}
                }
            }
        }

        if let Some(position) = position {
            label.set_position(geom_index, side, position);
            // JTS:   }
        }
    }

    // JTS:   /**
    // JTS:    * Update the IM with the contribution for the computed label for the EdgeStubs.
    // JTS:    */
    // JTS:   void updateIM(IntersectionMatrix im)
    // JTS:   {
    // JTS:     Edge.updateIM(label, im);
    // JTS:   }

    // JTS:   public void print(PrintStream out)
    // JTS:   {
    // JTS:     out.println("EdgeEndBundle--> Label: " + label);
    // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
    // JTS:       EdgeEnd ee = (EdgeEnd) it.next();
    // JTS:       ee.print(out);
    // JTS:       out.println();
    // JTS:     }
    // JTS:   }
    // JTS: }
}

/// An [`EdgeEndBundle`] whose topological relationships have been aggregated into a single
/// [`Label`].
///
/// `update_intersection_matrix` applies this aggregated topology to an `IntersectionMatrix`.
#[derive(Clone, Debug)]
pub(crate) struct LabeledEdgeEndBundle<F>
where
    F: GeoFloat,
{
    label: Label,
    edge_end_bundle: EdgeEndBundle<F>,
}

impl<F> LabeledEdgeEndBundle<F>
where
    F: GeoFloat,
{
    pub fn label(&self) -> &Label {
        &self.label
    }

    pub fn label_mut(&mut self) -> &mut Label {
        &mut self.label
    }

    pub fn update_intersection_matrix(&self, intersection_matrix: &mut IntersectionMatrix) {
        Edge::<F>::update_intersection_matrix(self.label(), intersection_matrix);
    }

    pub fn coordinate(&self) -> &Coordinate<F> {
        &self.edge_end_bundle.coordinate
    }
}
