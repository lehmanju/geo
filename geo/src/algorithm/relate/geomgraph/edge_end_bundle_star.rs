use super::{
    Dimensions, Direction, EdgeEnd, EdgeEndBundle, EdgeEndKey, GeometryGraph, IntersectionMatrix,
    LabeledEdgeEndBundle,
};
use crate::algorithm::coordinate_position::{CoordPos, CoordinatePosition};
use crate::{Coordinate, GeoFloat, GeometryCow};

// JTS: /**
// JTS:  * An ordered list of {@link EdgeEndBundle}s around a {@link RelateNode}.
// JTS:  * They are maintained in CCW order (starting with the positive x-axis) around the node
// JTS:  * for efficient lookup and topology building.
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class EdgeEndBundleStar
// JTS:   extends EdgeEndStar
// JTS: {
// JTS:   /**
// JTS:    * Creates a new empty EdgeEndBundleStar
// JTS:    */
// JTS:   public EdgeEndBundleStar() {
// JTS:   }
// JTS:
/// An ordered list of [`EdgeEndBundle`]s around a [`RelateNodeFactory::Node`].
///
/// They are maintained in CCW order (starting with the positive x-axis) around the node
/// for efficient lookup and topology building.
///
/// This is based on [JTS's `EdgeEndBundleStar` as of 1.18.1](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/operation/relate/EdgeEndBundleStar.java)
#[derive(Clone, Debug)]
pub(crate) struct EdgeEndBundleStar<F>
where
    F: GeoFloat,
{
    edge_map: std::collections::BTreeMap<EdgeEndKey<F>, EdgeEndBundle<F>>,
    point_in_area_location: Option<[CoordPos; 2]>,
}

#[derive(Clone, Debug)]
pub(crate) struct LabeledEdgeEndBundleStar<F>
where
    F: GeoFloat,
{
    edges: Vec<LabeledEdgeEndBundle<F>>,
}

impl<F: GeoFloat> LabeledEdgeEndBundleStar<F> {
    pub(crate) fn new(
        edges: Vec<LabeledEdgeEndBundle<F>>,
        graph_a: &GeometryGraph<F>,
        graph_b: &GeometryGraph<F>,
    ) -> Self {
        let mut labeled_bundle_star = Self { edges };
        labeled_bundle_star.compute_labeling(graph_a, graph_b);
        labeled_bundle_star
    }

    /// Compute a label for the star based on the labels of its EdgeEndBundles.
    fn compute_labeling(&mut self, graph_a: &GeometryGraph<F>, graph_b: &GeometryGraph<F>) {
        // JTS:     // Propagate side labels  around the edges in the star
        // JTS:     // for each parent Geometry
        // JTS: //Debug.print(this);
        // JTS:     propagateSideLabels(0);
        // JTS: //Debug.print(this);
        // JTS: //Debug.printIfWatch(this);
        // JTS:     propagateSideLabels(1);
        // JTS: //Debug.print(this);
        // JTS: //Debug.printIfWatch(this);
        self.propagate_side_labels(0);
        self.propagate_side_labels(1);
        // JTS:
        // JTS:     /**
        // JTS:      * If there are edges that still have null labels for a geometry
        // JTS:      * this must be because there are no area edges for that geometry incident on this node.
        // JTS:      * In this case, to label the edge for that geometry we must test whether the
        // JTS:      * edge is in the interior of the geometry.
        // JTS:      * To do this it suffices to determine whether the node for the edge is in the interior of an area.
        // JTS:      * If so, the edge has location INTERIOR for the geometry.
        // JTS:      * In all other cases (e.g. the node is on a line, on a point, or not on the geometry at all) the edge
        // JTS:      * has the location EXTERIOR for the geometry.
        // JTS:      * <p>
        // JTS:      * Note that the edge cannot be on the BOUNDARY of the geometry, since then
        // JTS:      * there would have been a parallel edge from the Geometry at this node also labelled BOUNDARY
        // JTS:      * and this edge would have been labelled in the previous step.
        // JTS:      * <p>
        // JTS:      * This code causes a problem when dimensional collapses are present, since it may try and
        // JTS:      * determine the location of a node where a dimensional collapse has occurred.
        // JTS:      * The point should be considered to be on the EXTERIOR
        // JTS:      * of the polygon, but locate() will return INTERIOR, since it is passed
        // JTS:      * the original Geometry, not the collapsed version.
        // JTS:      *
        // JTS:      * If there are incident edges which are Line edges labelled BOUNDARY,
        // JTS:      * then they must be edges resulting from dimensional collapses.
        // JTS:      * In this case the other edges can be labelled EXTERIOR for this Geometry.
        // JTS:      *
        // JTS:      * MD 8/11/01 - NOT TRUE!  The collapsed edges may in fact be in the interior of the Geometry,
        // JTS:      * which means the other edges should be labelled INTERIOR for this Geometry.
        // JTS:      * Not sure how solve this...  Possibly labelling needs to be split into several phases:
        // JTS:      * area label propagation, symLabel merging, then finally null label resolution.
        // JTS:      */
        // JTS:     boolean[] hasDimensionalCollapseEdge = { false, false };
        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        // JTS:       EdgeEnd e = (EdgeEnd) it.next();
        // JTS:       Label label = e.getLabel();
        // JTS:       for (int geomi = 0; geomi < 2; geomi++) {
        // JTS:         if (label.isLine(geomi) && label.getLocation(geomi) == Location.BOUNDARY)
        // JTS:           hasDimensionalCollapseEdge[geomi] = true;
        // JTS:       }
        // JTS:     }
        let mut has_dimensional_collapse_edge = [false, false];
        for edge_end in self.edges.iter() {
            let label = edge_end.label();
            for (geom_index, is_collapsed) in has_dimensional_collapse_edge.iter_mut().enumerate() {
                *is_collapsed = label.is_line(geom_index)
                    && label.on_position(geom_index) == Some(CoordPos::OnBoundary);
            }
        }
        // JTS: //Debug.print(this);
        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        // JTS:       EdgeEnd e = (EdgeEnd) it.next();
        // JTS:       Label label = e.getLabel();
        // JTS: //Debug.println(e);
        // JTS:       for (int geomi = 1; geomi < 2; geomi++) {
        for edge_end_bundle in &mut self.edges {
            let coord = *edge_end_bundle.coordinate();
            let label = edge_end_bundle.label_mut();
            for (geom_index, is_dimensionally_collapsed) in
                has_dimensional_collapse_edge.iter().enumerate()
            {
                // JTS:         if (label.isAnyNull(geomi)) {
                // JTS:           int loc = Location.NONE;
                // JTS:           if (hasDimensionalCollapseEdge[geomi]) {
                // JTS:             loc = Location.EXTERIOR;
                // JTS:           }
                // JTS:           else {
                // JTS:             Coordinate p = e.getCoordinate();
                // JTS:             loc = getLocation(geomi, p, geomGraph);
                // JTS:           }
                // JTS:           label.setAllLocationsIfNull(geomi, loc);
                // JTS:         }
                if label.is_any_empty(geom_index) {
                    let position: CoordPos = if *is_dimensionally_collapsed {
                        CoordPos::Outside
                    } else {
                        // PERF: In JTS this is memoized, but that gets a little tricky with rust's
                        // borrow checker. Let's wait to see if it's a hotspot.
                        let geometry = match geom_index {
                            0 => graph_a.geometry(),
                            1 => graph_b.geometry(),
                            _ => unreachable!("invalid geom_index"),
                        };
                        use crate::algorithm::dimensions::HasDimensions;
                        if geometry.dimensions() == Dimensions::TwoDimensional {
                            geometry.coordinate_position(&coord)
                        } else {
                            // if geometry is *not* an area, Coord is always Outside
                            CoordPos::Outside
                        }
                    };
                    label.set_all_positions_if_empty(geom_index, position);
                }
            }
        }
        debug!("edge_end_bundle_star: {:?}", self);
    }

    // JTS:   void propagateSideLabels(int geomIndex)
    // JTS:   {
    fn propagate_side_labels(&mut self, geom_index: usize) {
        // JTS:     // Since edges are stored in CCW order around the node,
        // JTS:     // As we move around the ring we move from the right to the left side of the edge
        // JTS:     int startLoc = Location.NONE ;
        let mut start_position = None;

        // JTS:     // initialize loc to location of last L side (if any)
        // JTS: //System.out.println("finding start location");
        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        // JTS:       EdgeEnd e = (EdgeEnd) it.next();
        // JTS:       Label label = e.getLabel();
        // JTS:       if (label.isArea(geomIndex) && label.getLocation(geomIndex, Position.LEFT) != Location.NONE)
        // JTS:         startLoc = label.getLocation(geomIndex, Position.LEFT);
        // JTS:     }
        for edge_ends in self.edge_end_bundles_iter() {
            let label = edge_ends.label();
            if label.is_geom_area(geom_index) {
                if let Some(position) = label.position(geom_index, Direction::Left) {
                    start_position = Some(position);
                }
            }
        }
        // JTS:     // no labelled sides found, so no labels to propagate
        // JTS:     if (startLoc == Location.NONE) return;
        if start_position.is_none() {
            return;
        }
        let mut current_position = start_position.unwrap();

        // JTS:     int currLoc = startLoc;
        // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
        for edge_ends in self.edge_end_bundles_iter_mut() {
            // JTS:       EdgeEnd e = (EdgeEnd) it.next();
            // JTS:       Label label = e.getLabel();
            let label = edge_ends.label_mut();
            // JTS:       // set null ON values to be in current location
            // JTS:       if (label.getLocation(geomIndex, Position.ON) == Location.NONE)
            // JTS:           label.setLocation(geomIndex, Position.ON, currLoc);
            if label.position(geom_index, Direction::On).is_none() {
                label.set_position(geom_index, Direction::On, current_position);
            }
            // JTS:       // set side labels (if any)
            // JTS:       if (label.isArea(geomIndex)) {
            if label.is_geom_area(geom_index) {
                // JTS:         int leftLoc   = label.getLocation(geomIndex, Position.LEFT);
                // JTS:         int rightLoc  = label.getLocation(geomIndex, Position.RIGHT);
                let left_position = label.position(geom_index, Direction::Left);
                let right_position = label.position(geom_index, Direction::Right);

                // JTS:         // if there is a right location, that is the next location to propagate
                // JTS:         if (rightLoc != Location.NONE) {
                // JTS: //Debug.print(rightLoc != currLoc, this);
                // JTS:           if (rightLoc != currLoc)
                // JTS:             throw new TopologyException("side location conflict", e.getCoordinate());
                // JTS:           if (leftLoc == Location.NONE) {
                // JTS:             Assert.shouldNeverReachHere("found single null side (at " + e.getCoordinate() + ")");
                // JTS:           }
                // JTS:           currLoc = leftLoc;
                if let Some(right_position) = right_position {
                    debug_assert!(right_position == current_position, "side_location conflict with coordinate: {:?}, right_location: {:?}, current_location: {:?}", edge_ends.coordinate(), right_position, current_position);
                    assert!(left_position.is_some(), "found single null side");
                    current_position = left_position.unwrap();
                } else {
                    // JTS:         }
                    // JTS:         else {
                    // JTS:           /** RHS is null - LHS must be null too.
                    // JTS:            *  This must be an edge from the other geometry, which has no location
                    // JTS:            *  labelling for this geometry.  This edge must lie wholly inside or outside
                    // JTS:            *  the other geometry (which is determined by the current location).
                    // JTS:            *  Assign both sides to be the current location.
                    // JTS:            */
                    // JTS:           Assert.isTrue(label.getLocation(geomIndex, Position.LEFT) == Location.NONE, "found single null side");
                    // JTS:           label.setLocation(geomIndex, Position.RIGHT, currLoc);
                    // JTS:           label.setLocation(geomIndex, Position.LEFT, currLoc);
                    // JTS:         }
                    // JTS:       }
                    // JTS:     }
                    // JTS:   }
                    debug_assert!(label.position(geom_index, Direction::Left).is_none());
                    label.set_position(geom_index, Direction::Right, current_position);
                    label.set_position(geom_index, Direction::Left, current_position);
                }
            }
        }
    }

    fn edge_end_bundles_iter(&self) -> impl Iterator<Item = &LabeledEdgeEndBundle<F>> {
        self.edges.iter()
    }

    fn edge_end_bundles_iter_mut(&mut self) -> impl Iterator<Item = &mut LabeledEdgeEndBundle<F>> {
        self.edges.iter_mut()
    }

    // JTS:   /**
    // JTS:    * Update the IM with the contribution for the EdgeStubs around the node.
    // JTS:    */
    // JTS:   void updateIM(IntersectionMatrix im)
    // JTS:   {
    // JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
    // JTS:       EdgeEndBundle esb = (EdgeEndBundle) it.next();
    // JTS:       esb.updateIM(im);
    // JTS:     }
    // JTS:   }
    pub fn update_intersection_matrix(&self, intersection_matrix: &mut IntersectionMatrix) {
        for edge_end_bundle in self.edge_end_bundles_iter() {
            edge_end_bundle.update_intersection_matrix(intersection_matrix);
            debug!(
                "updated intersection_matrix: {:?} from edge_end_bundle: {:?}",
                intersection_matrix, edge_end_bundle
            );
        }
    }
}

impl<F> EdgeEndBundleStar<F>
where
    F: GeoFloat,
{
    pub(crate) fn new() -> Self {
        EdgeEndBundleStar {
            edge_map: std::collections::BTreeMap::new(),
            point_in_area_location: None,
        }
    }

    // JTS:   /**
    // JTS:    * Insert a EdgeEnd in order in the list.
    // JTS:    * If there is an existing EdgeStubBundle which is parallel, the EdgeEnd is
    // JTS:    * added to the bundle.  Otherwise, a new EdgeEndBundle is created
    // JTS:    * to contain the EdgeEnd.
    // JTS:    * <br>
    // JTS:    */
    // JTS:   public void insert(EdgeEnd e)
    // JTS:   {
    // JTS:     EdgeEndBundle eb = (EdgeEndBundle) edgeMap.get(e);
    // JTS:     if (eb == null) {
    // JTS:       eb = new EdgeEndBundle(e);
    // JTS:       insertEdgeEnd(e, eb);
    // JTS:     }
    // JTS:     else {
    // JTS:       eb.insert(e);
    // JTS:     }
    // JTS:   }
    pub(crate) fn insert(&mut self, edge_end: EdgeEnd<F>) {
        let bundle = self
            .edge_map
            .entry(edge_end.key().clone())
            .or_insert_with(|| EdgeEndBundle::new(*edge_end.coordinate()));
        bundle.insert(edge_end);
    }

    fn edge_end_bundles_iter(&self) -> impl Iterator<Item = &EdgeEndBundle<F>> {
        self.edge_map.values()
    }

    fn edge_end_bundles_iter_mut(&mut self) -> impl Iterator<Item = &mut EdgeEndBundle<F>> {
        self.edge_map.values_mut()
    }

    /// Compute labeling for the star's EdgeEndBundles, and use them to compute an overall label
    /// for the star.
    ///
    /// Implementation Note: This is a bit of a divergence from JTS in two ways.
    ///
    /// Firstly, JTS doesn't leverage optionals, and sets nullable `Label`s, whereas here we convert
    /// to an explicitly labeled type to avoid unwrapping optionals later.
    ///
    /// Secondly, in JTS this functionality does not live directly on EdgeEndBundleStar, but rather
    /// on it's parent class [EdgeEndStar](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/geomgraph/EdgeEndStar.java#L117)
    ///
    /// Since we're only using this one subclass (EdgeEndBundleStar), we skip the
    /// complexity of mapping Java inheritance to Rust and implement this functionality
    /// on EdgeEndBundleStar directly.
    ///
    /// If/When we implement overlay operations we might consider extracting the superclass
    /// behavior.
    pub(crate) fn into_labeled(
        self,
        graph_a: &GeometryGraph<F>,
        graph_b: &GeometryGraph<F>,
    ) -> LabeledEdgeEndBundleStar<F> {
        debug!("edge_end_bundle_star: {:?}", self);
        // JTS:     computeEdgeEndLabels(geomGraph[0].getBoundaryNodeRule());
        let labeled_edges = self
            .edge_map
            .into_iter()
            .map(|(_k, v)| v.into_labeled())
            .collect();
        LabeledEdgeEndBundleStar::new(labeled_edges, graph_a, graph_b)
    }
}

// JTS:
// JTS:   /**
// JTS:    * A map which maintains the edges in sorted order around the node
// JTS:    */
// JTS:   protected Map edgeMap = new TreeMap();
// JTS:   /**
// JTS:    * A list of all outgoing edges in the result, in CCW order
// JTS:    */
// JTS:   protected List edgeList;
// JTS:   /**
// JTS:    * The location of the point for this star in Geometry i Areas
// JTS:    */
// JTS:   private int[] ptInAreaLocation = { Location.NONE, Location.NONE };
// JTS:
// JTS:   public EdgeEndStar()
// JTS:   {
// JTS:
// JTS:   }
// JTS:
// JTS:   /**
// JTS:    * Insert a EdgeEnd into this EdgeEndStar
// JTS:    */
// JTS:   abstract public void insert(EdgeEnd e);
// JTS:
// JTS:   /**
// JTS:    * Insert an EdgeEnd into the map, and clear the edgeList cache,
// JTS:    * since the list of edges has now changed
// JTS:    */
// JTS:   protected void insertEdgeEnd(EdgeEnd e, Object obj)
// JTS:   {
// JTS:     edgeMap.put(e, obj);
// JTS:     edgeList = null;  // edge list has changed - clear the cache
// JTS:   }
// JTS:
// JTS:   /**
// JTS:    * @return the coordinate for the node this star is based at
// JTS:    */
// JTS:   public Coordinate getCoordinate()
// JTS:   {
// JTS:     Iterator it = iterator();
// JTS:     if (! it.hasNext()) return null;
// JTS:     EdgeEnd e = (EdgeEnd) it.next();
// JTS:     return e.getCoordinate();
// JTS:   }
// JTS:   public int getDegree()
// JTS:   {
// JTS:     return edgeMap.size();
// JTS:   }
// JTS:
// JTS:   /**
// JTS:    * Iterator access to the ordered list of edges is optimized by
// JTS:    * copying the map collection to a list.  (This assumes that
// JTS:    * once an iterator is requested, it is likely that insertion into
// JTS:    * the map is complete).
// JTS:    */
// JTS:   public Iterator iterator()
// JTS:   {
// JTS:     return getEdges().iterator();
// JTS:   }

// JTS:   public List getEdges()
// JTS:   {
// JTS:     if (edgeList == null) {
// JTS:       edgeList = new ArrayList(edgeMap.values());
// JTS:     }
// JTS:     return edgeList;
// JTS:   }
// JTS:   public EdgeEnd getNextCW(EdgeEnd ee)
// JTS:   {
// JTS:     getEdges();
// JTS:     int i = edgeList.indexOf(ee);
// JTS:     int iNextCW = i - 1;
// JTS:     if (i == 0)
// JTS:       iNextCW = edgeList.size() - 1;
// JTS:     return (EdgeEnd) edgeList.get(iNextCW);
// JTS:   }

// JTS:   public void computeLabelling(GeometryGraph[] geomGraph)
// JTS:   {

// JTS:   private void computeEdgeEndLabels(BoundaryNodeRule boundaryNodeRule)
// JTS:   {
// JTS:     // Compute edge label for each EdgeEnd
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd ee = (EdgeEnd) it.next();
// JTS:       ee.computeLabel(boundaryNodeRule);
// JTS:     }
// JTS:   }

// JTS:   private int getLocation(int geomIndex, Coordinate p, GeometryGraph[] geom)
// JTS:   {
// JTS:     // compute location only on demand
// JTS:     if (ptInAreaLocation[geomIndex] == Location.NONE) {
// JTS:       ptInAreaLocation[geomIndex] = SimplePointInAreaLocator.locate(p, geom[geomIndex].getGeometry());
// JTS:     }
// JTS:     return ptInAreaLocation[geomIndex];
// JTS:   }

// JTS:   public boolean isAreaLabelsConsistent(GeometryGraph geomGraph)
// JTS:   {
// JTS:     computeEdgeEndLabels(geomGraph.getBoundaryNodeRule());
// JTS:     return checkAreaLabelsConsistent(0);
// JTS:   }
// JTS:
// JTS:   private boolean checkAreaLabelsConsistent(int geomIndex)
// JTS:   {
// JTS:     // Since edges are stored in CCW order around the node,
// JTS:     // As we move around the ring we move from the right to the left side of the edge
// JTS:     List edges = getEdges();
// JTS:     // if no edges, trivially consistent
// JTS:     if (edges.size() <= 0)
// JTS:       return true;
// JTS:     // initialize startLoc to location of last L side (if any)
// JTS:     int lastEdgeIndex = edges.size() - 1;
// JTS:     Label startLabel = ((EdgeEnd) edges.get(lastEdgeIndex)).getLabel();
// JTS:     int startLoc = startLabel.getLocation(geomIndex, Position.LEFT);
// JTS:     Assert.isTrue(startLoc != Location.NONE, "Found unlabelled area edge");
// JTS:
// JTS:     int currLoc = startLoc;
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       Label label = e.getLabel();
// JTS:       // we assume that we are only checking a area
// JTS:       Assert.isTrue(label.isArea(geomIndex), "Found non-area edge");
// JTS:       int leftLoc   = label.getLocation(geomIndex, Position.LEFT);
// JTS:       int rightLoc  = label.getLocation(geomIndex, Position.RIGHT);
// JTS: //System.out.println(leftLoc + " " + rightLoc);
// JTS: //Debug.print(this);
// JTS:       // check that edge is really a boundary between inside and outside!
// JTS:       if (leftLoc == rightLoc) {
// JTS:         return false;
// JTS:       }
// JTS:       // check side location conflict
// JTS:       //Assert.isTrue(rightLoc == currLoc, "side location conflict " + locStr);
// JTS:       if (rightLoc != currLoc) {
// JTS: //Debug.print(this);
// JTS:         return false;
// JTS:       }
// JTS:       currLoc = leftLoc;
// JTS:     }
// JTS:     return true;
// JTS:   }

// JTS:   public int findIndex(EdgeEnd eSearch)
// JTS:   {
// JTS:     iterator();   // force edgelist to be computed
// JTS:     for (int i = 0; i < edgeList.size(); i++ ) {
// JTS:       EdgeEnd e = (EdgeEnd) edgeList.get(i);
// JTS:       if (e == eSearch) return i;
// JTS:     }
// JTS:     return -1;
// JTS:   }
// JTS:
// JTS:   public void print(PrintStream out)
// JTS:   {
// JTS:     System.out.println("EdgeEndStar:   " + getCoordinate());
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       e.print(out);
// JTS:     }
// JTS:   }
// JTS:
// JTS:   public String toString()
// JTS:   {
// JTS:     StringBuffer buf = new StringBuffer();
// JTS:     buf.append("EdgeEndStar:   " + getCoordinate());
// JTS:     buf.append("\n");
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       buf.append(e);
// JTS:       buf.append("\n");
// JTS:     }
// JTS:     return buf.toString();
// JTS:   }
// JTS: }
