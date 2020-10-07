// JTS: import java.util.HashMap;
// JTS: import java.util.Iterator;
// JTS: import java.util.List;
// JTS: import java.util.Map;

use super::{
    index::{EdgeSetIntersector, SegmentIntersector, SimpleEdgeSetIntersector},
    CoordNode, CoordPos, Direction, Edge, Label, LineIntersector, PlanarGraph, TopologyPosition,
};

use crate::algorithm::dimensions::HasDimensions;
use crate::{Coordinate, GeoFloat, GeometryCow, Line, LineString, Point, Polygon};

use std::cell::RefCell;
use std::rc::Rc;

// JTS: import org.locationtech.jts.algorithm.BoundaryNodeRule;
// JTS: import org.locationtech.jts.algorithm.LineIntersector;
// JTS: import org.locationtech.jts.algorithm.Orientation;
// JTS: import org.locationtech.jts.algorithm.PointLocator;
// JTS: import org.locationtech.jts.algorithm.locate.IndexedPointInAreaLocator;
// JTS: import org.locationtech.jts.algorithm.locate.PointOnGeometryLocator;
// JTS: import org.locationtech.jts.geom.Coordinate;
// JTS: import org.locationtech.jts.geom.CoordinateArrays;
// JTS: import org.locationtech.jts.geom.Geometry;
// JTS: import org.locationtech.jts.geom.GeometryCollection;
// JTS: import org.locationtech.jts.geom.LineString;
// JTS: import org.locationtech.jts.geom.LinearRing;
// JTS: import org.locationtech.jts.geom.Location;
// JTS: import org.locationtech.jts.geom.MultiLineString;
// JTS: import org.locationtech.jts.geom.MultiPoint;
// JTS: import org.locationtech.jts.geom.MultiPolygon;
// JTS: import org.locationtech.jts.geom.Point;
// JTS: import org.locationtech.jts.geom.Polygon;
// JTS: import org.locationtech.jts.geom.Polygonal;
// JTS: import org.locationtech.jts.geomgraph.index.EdgeSetIntersector;
// JTS: import org.locationtech.jts.geomgraph.index.SegmentIntersector;
// JTS: import org.locationtech.jts.geomgraph.index.SimpleMCSweepLineIntersector;
// JTS: import org.locationtech.jts.util.Assert;

// JTS: /**
// JTS:  * A GeometryGraph is a graph that models a given Geometry
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class GeometryGraph
// JTS:   extends PlanarGraph
// JTS: {

/// The computation of the [`IntersectionMatrix`] relies on the use of a
/// structure called a "topology graph". The topology graph contains [nodes](CoordNode) and
/// [edges](Edge) corresponding to the nodes and line segments of a [`Geometry`]. Each
/// node and edge in the graph is labeled with its topological location
/// relative to the source geometry.
///
/// Note that there is no requirement that points of self-intersection be a
/// vertex. Thus to obtain a correct topology graph, [`Geometry`] must be
/// self-noded before constructing their graphs.
///
/// Two fundamental operations are supported by topology graphs:
///   - Computing the intersections between all the edges and nodes of a single graph
///   - Computing the intersections between the edges and nodes of two different graphs
///
/// GeometryGraph is based on [JTS's `GeomGraph` as of 1.18.1](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/geomgraph/GeometryGraph.java)
pub(crate) struct GeometryGraph<'a, F>
where
    F: GeoFloat,
{
    arg_index: usize,
    parent_geometry: &'a GeometryCow<'a, F>,
    use_boundary_determination_rule: bool,
    planar_graph: PlanarGraph<F>,
}

///  PlanarGraph delegations
///
/// In JTS, which is written in Java, GeometryGraph inherits from PlanarGraph. Here in Rust land we
/// use composition and delegation to the same effect.
impl<F> GeometryGraph<'_, F>
where
    F: GeoFloat,
{
    pub fn edges(&self) -> &[Rc<RefCell<Edge<F>>>] {
        self.planar_graph.edges()
    }

    pub fn insert_edge(&mut self, edge: Edge<F>) {
        self.planar_graph.insert_edge(edge)
    }

    pub fn is_boundary_node(&self, coord: Coordinate<F>) -> bool {
        self.planar_graph.is_boundary_node(self.arg_index, coord)
    }

    pub fn add_node_with_coordinate(&mut self, coord: Coordinate<F>) -> &mut CoordNode<F> {
        self.planar_graph.add_node_with_coordinate(coord)
    }

    pub fn nodes_iter(&self) -> impl Iterator<Item = &CoordNode<F>> {
        self.planar_graph.nodes.iter()
    }
}

impl<'a, F> GeometryGraph<'a, F>
where
    F: GeoFloat,
{
    // JTS:   public GeometryGraph(int argIndex, Geometry parentGeom)
    // JTS:   {
    // JTS:     this(argIndex, parentGeom,
    // JTS:          BoundaryNodeRule.OGC_SFS_BOUNDARY_RULE
    // JTS:          );
    // JTS:   }
    // JTS:
    // JTS:   public GeometryGraph(int argIndex, Geometry parentGeom, BoundaryNodeRule boundaryNodeRule) {
    // JTS:     this.argIndex = argIndex;
    // JTS:     this.parentGeom = parentGeom;
    // JTS:     this.boundaryNodeRule = boundaryNodeRule;
    // JTS:     if (parentGeom != null) {
    // JTS: //      precisionModel = parentGeom.getPrecisionModel();
    // JTS: //      SRID = parentGeom.getSRID();
    // JTS:       add(parentGeom);
    // JTS:     }
    // JTS:   }
    pub fn new(arg_index: usize, parent_geometry: &'a GeometryCow<F>) -> Self {
        let mut graph = GeometryGraph {
            arg_index,
            parent_geometry,
            use_boundary_determination_rule: true,
            planar_graph: PlanarGraph::new(),
        };
        graph.add_geometry(parent_geometry);
        graph
    }

    // JTS:
    // JTS:   /**
    // JTS:    * This constructor is used by clients that wish to add Edges explicitly,
    // JTS:    * rather than adding a Geometry.  (An example is BufferOp).
    // JTS:    */
    // JTS:   // no longer used
    // JTS: //  public GeometryGraph(int argIndex, PrecisionModel precisionModel, int SRID) {
    // JTS: //    this(argIndex, null);
    // JTS: //    this.precisionModel = precisionModel;
    // JTS: //    this.SRID = SRID;
    // JTS: //  }
    // JTS: //  public PrecisionModel getPrecisionModel()
    // JTS: //  {
    // JTS: //    return precisionModel;
    // JTS: //  }
    // JTS: //  public int getSRID() { return SRID; }
    // JTS:
    // JTS:   public boolean hasTooFewPoints() { return hasTooFewPoints; }
    // JTS:
    // JTS:   public Coordinate getInvalidPoint() { return invalidPoint; }
    // JTS:
    // JTS:   public Geometry getGeometry() { return parentGeom; }
    pub fn geometry(&self) -> &GeometryCow<F> {
        self.parent_geometry
    }

    // JTS: /**
    // JTS:  * This method implements the Boundary Determination Rule
    // JTS:  * for determining whether
    // JTS:  * a component (node or edge) that appears multiple times in elements
    // JTS:  * of a MultiGeometry is in the boundary or the interior of the Geometry
    // JTS:  * <br>
    // JTS:  * The SFS uses the "Mod-2 Rule", which this function implements
    // JTS:  * <br>
    // JTS:  * An alternative (and possibly more intuitive) rule would be
    // JTS:  * the "At Most One Rule":
    // JTS:  *    isInBoundary = (componentCount == 1)
    // JTS:  */
    // JTS: /*
    // JTS:   public static boolean isInBoundary(int boundaryCount)
    // JTS:   {
    // JTS:     // the "Mod-2 Rule"
    // JTS:     return boundaryCount % 2 == 1;
    // JTS:   }
    // JTS:   public static int determineBoundary(int boundaryCount)
    // JTS:   {
    // JTS:     return isInBoundary(boundaryCount) ? Location.BOUNDARY : Location.INTERIOR;
    // JTS:   }
    // JTS: */
    // JTS:
    // JTS:   public static int determineBoundary(BoundaryNodeRule boundaryNodeRule, int boundaryCount)
    // JTS:   {
    // JTS:     return boundaryNodeRule.isInBoundary(boundaryCount)
    // JTS:         ? Location.BOUNDARY : Location.INTERIOR;
    // JTS:   }

    /// Determine whether a component (node or edge) that appears multiple times in elements
    /// of a Multi-Geometry is in the boundary or the interior of the Geometry
    pub fn determine_boundary(boundary_count: usize) -> CoordPos {
        // For now, we only support the SFS "Mod-2 Rule"
        // We could make this configurable if we wanted to support alternative boundary rules.
        if boundary_count % 2 == 1 {
            CoordPos::OnBoundary
        } else {
            CoordPos::Inside
        }
    }

    // JTS:   private Geometry parentGeom;
    // JTS:
    // JTS:   /**
    // JTS:    * The lineEdgeMap is a map of the linestring components of the
    // JTS:    * parentGeometry to the edges which are derived from them.
    // JTS:    * This is used to efficiently perform findEdge queries
    // JTS:    */
    // JTS:   private Map lineEdgeMap = new HashMap();
    // JTS:
    // JTS:   private BoundaryNodeRule boundaryNodeRule = null;
    // JTS:
    // JTS:   /**
    // JTS:    * If this flag is true, the Boundary Determination Rule will used when deciding
    // JTS:    * whether nodes are in the boundary or not
    // JTS:    */
    // JTS:   private boolean useBoundaryDeterminationRule = true;
    // JTS:   private int argIndex;  // the index of this geometry as an argument to a spatial function (used for labelling)
    // JTS:   private Collection boundaryNodes;
    // JTS:   private boolean hasTooFewPoints = false;
    // JTS:   private Coordinate invalidPoint = null;
    // JTS:
    // JTS:   private PointOnGeometryLocator areaPtLocator = null;
    // JTS:   // for use if geometry is not Polygonal
    // JTS:   private final PointLocator ptLocator = new PointLocator();
    // JTS:
    // JTS:   private EdgeSetIntersector createEdgeSetIntersector()
    // JTS:   {
    fn create_edge_set_intersector() -> Box<dyn EdgeSetIntersector<F>> {
        // JTS:   // various options for computing intersections, from slowest to fastest
        // JTS:
        // JTS:   //private EdgeSetIntersector esi = new SimpleEdgeSetIntersector();
        // JTS:   //private EdgeSetIntersector esi = new MonotoneChainIntersector();
        // JTS:   //private EdgeSetIntersector esi = new NonReversingChainIntersector();
        // JTS:   //private EdgeSetIntersector esi = new SimpleSweepLineIntersector();
        // JTS:   //private EdgeSetIntersector esi = new MCSweepLineIntersector();
        // JTS:
        // JTS:     //return new SimpleEdgeSetIntersector();
        // PERF: faster algorithms exist. This one was chosen for simplicity of implementation and
        //       debugging
        Box::new(SimpleEdgeSetIntersector::new())
        // JTS:     return new SimpleMCSweepLineIntersector();
        // JTS:   }
    }

    // JTS:   public BoundaryNodeRule getBoundaryNodeRule() { return boundaryNodeRule; }
    // JTS:
    // JTS:   public Collection getBoundaryNodes()
    // JTS:   {
    // JTS:     if (boundaryNodes == null)
    // JTS:       boundaryNodes = nodes.getBoundaryNodes(argIndex);
    // JTS:     return boundaryNodes;
    // JTS:   }
    fn boundary_nodes(&self) -> impl Iterator<Item = &CoordNode<F>> {
        self.planar_graph.boundary_nodes(self.arg_index)
    }
    // JTS:
    // JTS:   public Coordinate[] getBoundaryPoints()
    // JTS:   {
    // JTS:     Collection coll = getBoundaryNodes();
    // JTS:     Coordinate[] pts = new Coordinate[coll.size()];
    // JTS:     int i = 0;
    // JTS:     for (Iterator it = coll.iterator(); it.hasNext(); ) {
    // JTS:       Node node = (Node) it.next();
    // JTS:       pts[i++] = node.getCoordinate().copy();
    // JTS:     }
    // JTS:     return pts;
    // JTS:   }
    // JTS:
    // JTS:   public Edge findEdge(LineString line)
    // JTS:   {
    // JTS:     return (Edge) lineEdgeMap.get(line);
    // JTS:   }
    // JTS:
    // JTS:   public void computeSplitEdges(List edgelist)
    // JTS:   {
    // JTS:     for (Iterator i = edges.iterator(); i.hasNext(); ) {
    // JTS:       Edge e = (Edge) i.next();
    // JTS:       e.eiList.addSplitEdges(edgelist);
    // JTS:     }
    // JTS:   }

    // JTS:   private void add(Geometry g)
    // JTS:   {
    pub fn add_geometry(&mut self, geometry: &GeometryCow<F>) {
        // JTS:     if (g.isEmpty()) return;
        if geometry.is_empty() {
            return;
        }
        // JTS:
        // JTS:     if (g instanceof Polygon)                 addPolygon((Polygon) g);
        // JTS:                         // LineString also handles LinearRings
        // JTS:     else if (g instanceof LineString)         addLineString((LineString) g);
        // JTS:     else if (g instanceof Point)              addPoint((Point) g);
        // JTS:     else if (g instanceof MultiPoint)         addCollection((MultiPoint) g);
        // JTS:     else if (g instanceof MultiLineString)    addCollection((MultiLineString) g);
        // JTS:     else if (g instanceof MultiPolygon)       addCollection((MultiPolygon) g);
        // JTS:     else if (g instanceof GeometryCollection) addCollection((GeometryCollection) g);
        // JTS:     else  throw new UnsupportedOperationException(g.getClass().getName());
        // JTS:   }
        match geometry {
            GeometryCow::Line(line) => self.add_line(line),
            GeometryCow::Rect(rect) => {
                // PERF: avoid this conversion/clone?
                self.add_polygon(&rect.to_polygon());
            }
            GeometryCow::Point(point) => {
                self.add_point(point);
            }
            GeometryCow::Polygon(polygon) => self.add_polygon(polygon),
            GeometryCow::Triangle(triangle) => {
                // PERF: avoid this conversion/clone?
                self.add_polygon(&Polygon::from(triangle.clone().into_owned()));
            }
            GeometryCow::LineString(line_string) => self.add_line_string(line_string),
            GeometryCow::MultiPoint(multi_point) => {
                for point in &multi_point.0 {
                    self.add_point(point);
                }
            }
            GeometryCow::MultiPolygon(multi_polygon) => {
                // JTS:     // check if this Geometry should obey the Boundary Determination Rule
                // JTS:     // all collections except MultiPolygons obey the rule
                // JTS:     if (g instanceof MultiPolygon)
                // JTS:       useBoundaryDeterminationRule = false;
                // check if this Geometry should obey the Boundary Determination Rule
                // all collections except MultiPolygons obey the rule
                self.use_boundary_determination_rule = false;
                for polygon in &multi_polygon.0 {
                    self.add_polygon(polygon);
                }
            }
            GeometryCow::MultiLineString(multi_line_string) => {
                for line_string in &multi_line_string.0 {
                    self.add_line_string(line_string);
                }
            }
            GeometryCow::GeometryCollection(geometry_collection) => {
                for geometry in geometry_collection.iter() {
                    self.add_geometry(&GeometryCow::from(geometry));
                }
            }
        }
    }

    // JTS:   private void addCollection(GeometryCollection gc)
    // JTS:   {
    // JTS:     for (int i = 0; i < gc.getNumGeometries(); i++) {
    // JTS:       Geometry g = gc.getGeometryN(i);
    // JTS:       add(g);
    // JTS:     }
    // JTS:   }
    // JTS:   /**
    // JTS:    * Add a Point to the graph.
    // JTS:    */
    // JTS:   private void addPoint(Point p)
    // JTS:   {
    // JTS:     Coordinate coord = p.getCoordinate();
    // JTS:     insertPoint(argIndex, coord, Location.INTERIOR);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Adds a polygon ring to the graph.
    // JTS:    * Empty rings are ignored.
    // JTS:    *
    // JTS:    * The left and right topological location arguments assume that the ring is oriented CW.
    // JTS:    * If the ring is in the opposite orientation,
    // JTS:    * the left and right locations must be interchanged.
    // JTS:    */
    // JTS:   private void addPolygonRing(LinearRing lr, int cwLeft, int cwRight)
    // JTS:   {
    fn add_polygon_ring(
        &mut self,
        linear_ring: &LineString<F>,
        cw_left: CoordPos,
        cw_right: CoordPos,
    ) {
        debug_assert!(linear_ring.is_closed());
        // JTS:   	// don't bother adding empty holes
        // JTS:   	if (lr.isEmpty()) return;
        if linear_ring.is_empty() {
            return;
        }

        // JTS:     Coordinate[] coord = CoordinateArrays.removeRepeatedPoints(lr.getCoordinates());
        let mut coords: Vec<Coordinate<F>> = vec![];
        // remove repeated coords
        for coord in &linear_ring.0 {
            if coords.last() != Some(coord) {
                coords.push(*coord)
            }
        }

        // JTS:     if (coord.length < 4) {
        // JTS:       hasTooFewPoints = true;
        // JTS:       invalidPoint = coord[0];
        // JTS:       return;
        // JTS:     }
        if coords.len() < 4 {
            // TODO: we could return an Err here, but this has ramifications for how we can
            // use this code in other operations - do we want all our methods, like `contains` to
            // return a Result?
            warn!("encountered invalid ring, which has undefined results");
        }
        let first_point = coords[0];

        // JTS:
        // JTS:     int left  = cwLeft;
        // JTS:     int right = cwRight;
        // JTS:     if (Orientation.isCCW(coord)) {
        // JTS:       left = cwRight;
        // JTS:       right = cwLeft;
        // JTS:     }
        use crate::algorithm::winding_order::{Winding, WindingOrder};
        let (left, right) = match linear_ring.winding_order() {
            Some(WindingOrder::Clockwise) => (cw_left, cw_right),
            Some(WindingOrder::CounterClockwise) => (cw_right, cw_left),
            None => {
                warn!("polygon had no winding order. Results are undefined.");
                (cw_left, cw_right)
            }
        };

        // JTS:     Edge e = new Edge(coord,
        // JTS:                         new Label(argIndex, Location.BOUNDARY, left, right));
        // JTS:     lineEdgeMap.put(lr, e);
        // JTS:     insertEdge(e);
        let edge = Edge::new(
            coords,
            Label::new(
                self.arg_index,
                TopologyPosition::area(CoordPos::OnBoundary, left, right),
            ),
        );
        self.insert_edge(edge);

        // JTS:     // insert the endpoint as a node, to mark that it is on the boundary
        // JTS:     insertPoint(argIndex, coord[0], Location.BOUNDARY);
        // insert the endpoint as a node, to mark that it is on the boundary
        self.insert_point(self.arg_index, first_point, CoordPos::OnBoundary);
        // JTS:   }
    }

    // JTS:   private void addPolygon(Polygon p)
    // JTS:   {
    fn add_polygon(&mut self, polygon: &Polygon<F>) {
        // JTS:     addPolygonRing(
        // JTS:             p.getExteriorRing(),
        // JTS:             Location.EXTERIOR,
        // JTS:             Location.INTERIOR);
        self.add_polygon_ring(polygon.exterior(), CoordPos::Outside, CoordPos::Inside);
        // JTS:     for (int i = 0; i < p.getNumInteriorRing(); i++) {
        // JTS:     	LinearRing hole = p.getInteriorRingN(i);
        // JTS:
        // JTS:       // Holes are topologically labelled opposite to the shell, since
        // JTS:       // the interior of the polygon lies on their opposite side
        // JTS:       // (on the left, if the hole is oriented CW)
        // JTS:       addPolygonRing(
        // JTS:       		hole,
        // JTS:           Location.INTERIOR,
        // JTS:           Location.EXTERIOR);
        // JTS:     }
        // JTS:   }
        // Holes are topologically labeled opposite to the shell, since
        // the interior of the polygon lies on their opposite side
        // (on the left, if the hole is oriented CW)
        for hole in polygon.interiors() {
            self.add_polygon_ring(hole, CoordPos::Inside, CoordPos::Outside)
        }
    }

    // JTS:   private void addLineString(LineString line)
    // JTS:   {
    fn add_line_string(&mut self, line_string: &LineString<F>) {
        if line_string.is_empty() {
            return;
        }

        // JTS:     Coordinate[] coord = CoordinateArrays.removeRepeatedPoints(line.getCoordinates());
        let mut coords: Vec<Coordinate<F>> = vec![];
        for coord in &line_string.0 {
            if coords.last() != Some(coord) {
                coords.push(*coord)
            }
        }

        // JTS:     if (coord.length < 2) {
        // JTS:       hasTooFewPoints = true;
        // JTS:       invalidPoint = coord[0];
        // JTS:       return;
        // JTS:     }
        if coords.len() < 2 {
            // TODO: we could return an Err here, but this has ramifications for how we can
            // use this code in other operations - do we want all our methods, like `contains` to
            // return a Result?
            warn!("encountered invalid linestring, which has undefined results");
        }
        self.insert_boundary_point(*coords.first().unwrap());
        self.insert_boundary_point(*coords.last().unwrap());

        // JTS:
        // JTS:     // add the edge for the LineString
        // JTS:     // line edges do not have locations for their left and right sides
        // JTS:     Edge e = new Edge(coord, new Label(argIndex, Location.INTERIOR));
        // JTS:     lineEdgeMap.put(line, e);
        // JTS:     insertEdge(e);
        let edge = Edge::new(
            coords,
            Label::new(
                self.arg_index,
                TopologyPosition::line_or_point(CoordPos::Inside),
            ),
        );
        self.insert_edge(edge);

        // JTS:     /**
        // JTS:      * Add the boundary points of the LineString, if any.
        // JTS:      * Even if the LineString is closed, add both points as if they were endpoints.
        // JTS:      * This allows for the case that the node already exists and is a boundary point.
        // JTS:      */
        // JTS:     Assert.isTrue(coord.length >= 2, "found LineString with single point");
        // JTS:     insertBoundaryPoint(argIndex, coord[0]);
        // JTS:     insertBoundaryPoint(argIndex, coord[coord.length - 1]);
        // JTS:   }
    }

    fn add_line(&mut self, line: &Line<F>) {
        self.insert_boundary_point(line.start);
        self.insert_boundary_point(line.end);

        let edge = Edge::new(
            vec![line.start, line.end],
            Label::new(
                self.arg_index,
                TopologyPosition::line_or_point(CoordPos::Inside),
            ),
        );

        self.insert_edge(edge);
    }

    // JTS:
    // JTS:   /**
    // JTS:    * Add an Edge computed externally.  The label on the Edge is assumed
    // JTS:    * to be correct.
    // JTS:    */
    // JTS:   public void addEdge(Edge e)
    // JTS:   {
    // JTS:     insertEdge(e);
    // JTS:     Coordinate[] coord = e.getCoordinates();
    // JTS:     // insert the endpoint as a node, to mark that it is on the boundary
    // JTS:     insertPoint(argIndex, coord[0], Location.BOUNDARY);
    // JTS:     insertPoint(argIndex, coord[coord.length - 1], Location.BOUNDARY);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Add a point computed externally.  The point is assumed to be a
    // JTS:    * Point Geometry part, which has a location of INTERIOR.
    // JTS:    */
    // JTS:   public void addPoint(Coordinate pt)
    // JTS:   {
    // JTS:     insertPoint(argIndex, pt, Location.INTERIOR);
    // JTS:   }
    /// Add a point computed externally.  The point is assumed to be a
    /// Point Geometry part, which has a location of INTERIOR.
    fn add_point(&mut self, point: &Point<F>) {
        self.insert_point(self.arg_index, point.clone().into(), CoordPos::Inside);
    }

    // JTS:   /**
    // JTS:    * Compute self-nodes, taking advantage of the Geometry type to
    // JTS:    * minimize the number of intersection tests.  (E.g. rings are
    // JTS:    * not tested for self-intersection, since they are assumed to be valid).
    // JTS:    *
    // JTS:    * @param li the LineIntersector to use
    // JTS:    * @param computeRingSelfNodes if <code>false</code>, intersection checks are optimized to not test rings for self-intersection
    // JTS:    * @return the computed SegmentIntersector containing information about the intersections found
    // JTS:    */
    // JTS:   public SegmentIntersector computeSelfNodes(LineIntersector li, boolean computeRingSelfNodes)
    // JTS:   {
    // JTS: 	  return computeSelfNodes(li, computeRingSelfNodes, false);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Compute self-nodes, taking advantage of the Geometry type to
    // JTS:    * minimize the number of intersection tests.  (E.g. rings are
    // JTS:    * not tested for self-intersection, since they are assumed to be valid).
    // JTS:    *
    // JTS:    * @param li the LineIntersector to use
    // JTS:    * @param computeRingSelfNodes if <code>false</code>, intersection checks are optimized to not test rings for self-intersection
    // JTS:    * @param isDoneIfProperInt short-circuit the intersection computation if a proper intersection is found
    // JTS:    * @return the computed SegmentIntersector containing information about the intersections found
    // JTS:    */
    // JTS:   public SegmentIntersector computeSelfNodes(LineIntersector li, boolean computeRingSelfNodes, boolean isDoneIfProperInt)
    // JTS:   {
    /// Compute self-nodes, taking advantage of the Geometry type to minimize the number of
    /// intersection tests.  (E.g. rings are not tested for self-intersection, since they are
    /// assumed to be valid).
    ///
    /// `li` the [`LineIntersector`] to use to determine intersection
    pub fn compute_self_nodes(
        &mut self,
        line_intersector: Box<dyn LineIntersector<F>>,
    ) -> SegmentIntersector<F> {
        // JTS:     SegmentIntersector si = new SegmentIntersector(li, true, false);
        let mut segment_intersector = SegmentIntersector::new(line_intersector, true);

        // JTS:     si.setIsDoneIfProperInt(isDoneIfProperInt);

        // JTS:     EdgeSetIntersector esi = createEdgeSetIntersector();
        let mut edge_set_intersector = Self::create_edge_set_intersector();

        // JTS:     // optimize intersection search for valid Polygons and LinearRings
        // JTS:     boolean isRings = parentGeom instanceof LinearRing
        // JTS: 			|| parentGeom instanceof Polygon
        // JTS: 			|| parentGeom instanceof MultiPolygon;
        // JTS:     boolean computeAllSegments = computeRingSelfNodes || ! isRings;
        // optimize intersection search for valid Polygons and LinearRings
        let is_rings = match self.geometry() {
            GeometryCow::LineString(ls) => ls.is_closed(),
            GeometryCow::MultiLineString(ls) => ls.is_closed(),
            GeometryCow::Polygon(_) | GeometryCow::MultiPolygon(_) => true,
            _ => false,
        };
        let check_for_self_intersecting_edges = !is_rings;

        // JTS:     esi.computeIntersections(edges, si, computeAllSegments);
        edge_set_intersector.compute_intersections_within_set(
            self.edges(),
            check_for_self_intersecting_edges,
            &mut segment_intersector,
        );

        // JTS:     //System.out.println("SegmentIntersector # tests = " + si.numTests);
        // JTS:     addSelfIntersectionNodes(argIndex);
        self.add_self_intersection_nodes();

        // JTS:     return si;
        // JTS:   }
        segment_intersector
    }

    // JTS:   public SegmentIntersector computeEdgeIntersections(
    // JTS:     GeometryGraph g,
    // JTS:     LineIntersector li,
    // JTS:     boolean includeProper)
    // JTS:   {
    pub fn compute_edge_intersections(
        &self,
        other: &GeometryGraph<F>,
        line_intersector: Box<dyn LineIntersector<F>>,
    ) -> SegmentIntersector<F> {
        // JTS:     SegmentIntersector si = new SegmentIntersector(li, includeProper, true);
        // JTS:     si.setBoundaryNodes(this.getBoundaryNodes(), g.getBoundaryNodes());
        let mut segment_intersector = SegmentIntersector::new(line_intersector, false);
        segment_intersector.set_boundary_nodes(
            self.boundary_nodes().into_iter().cloned().collect(),
            other.boundary_nodes().into_iter().cloned().collect(),
        );

        // JTS:
        // JTS:     EdgeSetIntersector esi = createEdgeSetIntersector();
        // JTS:     esi.computeIntersections(edges, g.edges, si);
        let mut edge_set_intersector = Self::create_edge_set_intersector();
        edge_set_intersector.compute_intersections_between_sets(
            self.edges(),
            other.edges(),
            &mut segment_intersector,
        );

        // JTS: /*
        // JTS: for (Iterator i = g.edges.iterator(); i.hasNext();) {
        // JTS: Edge e = (Edge) i.next();
        // JTS: Debug.print(e.getEdgeIntersectionList());
        // JTS: }
        // JTS: */
        // JTS:     return si;
        segment_intersector
        // JTS:   }
    }

    // JTS:   private void insertPoint(int argIndex, Coordinate coord, int onLocation)
    // JTS:   {
    // JTS:     Node n = nodes.addNode(coord);
    // JTS:     Label lbl = n.getLabel();
    // JTS:     if (lbl == null) {
    // JTS:       n.label = new Label(argIndex, onLocation);
    // JTS:     }
    // JTS:     else
    // JTS:       lbl.setLocation(argIndex, onLocation);
    // JTS:   }
    fn insert_point(&mut self, arg_index: usize, coord: Coordinate<F>, position: CoordPos) {
        let node: &mut CoordNode<F> = self.add_node_with_coordinate(coord);
        node.label_mut().set_on_position(arg_index, position);
    }

    // JTS:   /**
    // JTS:    * Adds candidate boundary points using the current {@link BoundaryNodeRule}.
    // JTS:    * This is used to add the boundary
    // JTS:    * points of dim-1 geometries (Curves/MultiCurves).
    // JTS:    */
    // JTS:   private void insertBoundaryPoint(int argIndex, Coordinate coord)
    // JTS:   {
    /// Add the boundary points of 1-dim (line) geometries.
    fn insert_boundary_point(&mut self, coord: Coordinate<F>) {
        // JTS:     Node n = nodes.addNode(coord);
        let arg_index = self.arg_index;
        let node: &mut CoordNode<F> = self.add_node_with_coordinate(coord);

        // JTS:     // nodes always have labels
        // JTS:     Label lbl = n.getLabel();
        let label: &mut Label = node.label_mut();

        // JTS:     // the new point to insert is on a boundary
        // JTS:     int boundaryCount = 1;
        // JTS:     // determine the current location for the point (if any)
        // JTS:     int loc = Location.NONE;
        // JTS:     loc = lbl.getLocation(argIndex, Position.ON);
        // JTS:     if (loc == Location.BOUNDARY) boundaryCount++;

        // determine the current location for the point (if any)
        let boundary_count = {
            let prev_boundary_count =
                if Some(CoordPos::OnBoundary) == label.position(arg_index, Direction::On) {
                    1
                } else {
                    0
                };
            prev_boundary_count + 1
        };

        // JTS:     // determine the boundary status of the point according to the Boundary Determination Rule
        // JTS:     int newLoc = determineBoundary(boundaryNodeRule, boundaryCount);
        // JTS:     lbl.setLocation(argIndex, newLoc);
        let new_position = Self::determine_boundary(boundary_count);
        label.set_on_position(arg_index, new_position);
        // JTS:   }
    }
    // JTS:
    // JTS:   private void addSelfIntersectionNodes(int argIndex)
    // JTS:   {
    fn add_self_intersection_nodes(&mut self) {
        // JTS:     for (Iterator i = edges.iterator(); i.hasNext(); ) {
        // JTS:       Edge e = (Edge) i.next();
        // JTS:       int eLoc = e.getLabel().getLocation(argIndex);
        // JTS:       for (Iterator eiIt = e.eiList.iterator(); eiIt.hasNext(); ) {
        // JTS:         EdgeIntersection ei = (EdgeIntersection) eiIt.next();
        // JTS:         addSelfIntersectionNode(argIndex, ei.coord, eLoc);
        // JTS:       }
        // JTS:     }
        // JTS:   }

        let positions_and_intersections: Vec<(CoordPos, Vec<Coordinate<F>>)> = self
            .edges()
            .iter()
            .map(|cell| cell.borrow())
            .map(|edge| {
                let position = edge
                    .label()
                    .on_position(self.arg_index)
                    .expect("all edge labels should have an `on` position by now");
                let coordinates = edge
                    .edge_intersections()
                    .iter()
                    .map(|edge_intersection| edge_intersection.coordinate());

                (position, coordinates.collect())
            })
            .collect();

        for (position, edge_intersection_coordinates) in positions_and_intersections {
            for coordinate in edge_intersection_coordinates {
                self.add_self_intersection_node(coordinate, position)
            }
        }
    }

    // JTS:   /**
    // JTS:    * Add a node for a self-intersection.
    // JTS:    * If the node is a potential boundary node (e.g. came from an edge which
    // JTS:    * is a boundary) then insert it as a potential boundary node.
    // JTS:    * Otherwise, just add it as a regular node.
    // JTS:    */
    // JTS:   private void addSelfIntersectionNode(int argIndex, Coordinate coord, int loc)
    // JTS:   {
    /// Add a node for a self-intersection.
    ///
    /// If the node is a potential boundary node (e.g. came from an edge which is a boundary), then
    /// insert it as a potential boundary node.  Otherwise, just add it as a regular node.
    fn add_self_intersection_node(&mut self, coord: Coordinate<F>, position: CoordPos) {
        // JTS:     // if this node is already a boundary node, don't change it
        // JTS:     if (isBoundaryNode(argIndex, coord)) return;
        // if this node is already a boundary node, don't change it
        if self.is_boundary_node(coord) {
            return;
        }

        // JTS:     if (loc == Location.BOUNDARY && useBoundaryDeterminationRule)
        // JTS:         insertBoundaryPoint(argIndex, coord);
        // JTS:     else
        // JTS:       insertPoint(argIndex, coord, loc);
        if position == CoordPos::OnBoundary && self.use_boundary_determination_rule {
            self.insert_boundary_point(coord)
        } else {
            self.insert_point(self.arg_index, coord, position)
        }
        // JTS:   }
    }
    // JTS:
    // JTS:   // MD - experimental for now
    // JTS:   /**
    // JTS:    * Determines the {@link Location} of the given {@link Coordinate}
    // JTS:    * in this geometry.
    // JTS:    *
    // JTS:    * @param pt the point to test
    // JTS:    * @return the location of the point in the geometry
    // JTS:    */
    // JTS:   public int locate(Coordinate pt)
    // JTS:   {
    // JTS:   	if (parentGeom instanceof Polygonal && parentGeom.getNumGeometries() > 50) {
    // JTS:   		// lazily init point locator
    // JTS:   		if (areaPtLocator == null) {
    // JTS:   			areaPtLocator = new IndexedPointInAreaLocator(parentGeom);
    // JTS:   		}
    // JTS:   		return areaPtLocator.locate(pt);
    // JTS:   	}
    // JTS:   	return ptLocator.locate(pt, parentGeom);
    // JTS:   }
    // JTS: }
}
