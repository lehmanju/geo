use super::{
    node_map::{NodeFactory, NodeMap},
    CoordNode, CoordPos, Edge, Label,
};
use crate::{Coordinate, GeoFloat};

use std::cell::RefCell;
use std::rc::Rc;

// JTS: import org.locationtech.jts.algorithm.Orientation;
// JTS: import org.locationtech.jts.geom.Coordinate;
// JTS: import org.locationtech.jts.geom.Location;
// JTS:
// JTS: /**
// JTS:  * The computation of the <code>IntersectionMatrix</code> relies on the use of a structure
// JTS:  * called a "topology graph".  The topology graph contains nodes and edges
// JTS:  * corresponding to the nodes and line segments of a <code>Geometry</code>. Each
// JTS:  * node and edge in the graph is labeled with its topological location relative to
// JTS:  * the source geometry.
// JTS:  * <P>
// JTS:  * Note that there is no requirement that points of self-intersection be a vertex.
// JTS:  * Thus to obtain a correct topology graph, <code>Geometry</code>s must be
// JTS:  * self-noded before constructing their graphs.
// JTS:  * <P>
// JTS:  * Two fundamental operations are supported by topology graphs:
// JTS:  * <UL>
// JTS:  *   <LI>Computing the intersections between all the edges and nodes of a single graph
// JTS:  *   <LI>Computing the intersections between the edges and nodes of two different graphs
// JTS:  * </UL>
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: public class PlanarGraph
// JTS: {
pub(crate) struct PlanarGraphNode;

/// The basic node constructor does not allow for incident edges
impl<F> NodeFactory<F> for PlanarGraphNode
where
    F: GeoFloat,
{
    type Node = CoordNode<F>;
    fn create_node(coordinate: Coordinate<F>) -> Self::Node {
        CoordNode::new(coordinate)
    }
}

pub(crate) struct PlanarGraph<F: GeoFloat> {
    pub(crate) nodes: NodeMap<F, PlanarGraphNode>,
    edges: Vec<Rc<RefCell<Edge<F>>>>,
}

impl<F: GeoFloat> PlanarGraph<F> {
    // JTS:   /**
    // JTS:    * For nodes in the Collection, link the DirectedEdges at the node that are in the result.
    // JTS:    * This allows clients to link only a subset of nodes in the graph, for
    // JTS:    * efficiency (because they know that only a subset is of interest).
    // JTS:    */
    // JTS:   public static void linkResultDirectedEdges(Collection nodes)
    // JTS:   {
    // JTS:     for (Iterator nodeit = nodes.iterator(); nodeit.hasNext(); ) {
    // JTS:       Node node = (Node) nodeit.next();
    // JTS:       ((DirectedEdgeStar) node.getEdges()).linkResultDirectedEdges();
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   protected List edges        = new ArrayList();
    pub fn edges(&self) -> &[Rc<RefCell<Edge<F>>>] {
        &self.edges
    }

    // JTS:   protected NodeMap nodes;
    // JTS:   protected List edgeEndList  = new ArrayList();
    // JTS:
    // JTS:   public PlanarGraph(NodeFactory nodeFact) {
    // JTS:     nodes = new NodeMap(nodeFact);
    // JTS:   }
    // JTS:   public PlanarGraph() {
    // JTS:     nodes = new NodeMap(new NodeFactory());
    // JTS:   }
    pub fn new() -> Self {
        PlanarGraph {
            nodes: NodeMap::new(),
            edges: vec![],
        }
    }

    // JTS:   public Iterator getEdgeIterator() { return edges.iterator(); }
    // JTS:   public Collection getEdgeEnds() { return edgeEndList; }

    // JTS:   public boolean isBoundaryNode(int geomIndex, Coordinate coord)
    // JTS:   {
    // JTS:     Node node = nodes.find(coord);
    // JTS:     if (node == null) return false;
    // JTS:     Label label = node.getLabel();
    // JTS:     if (label != null && label.getLocation(geomIndex) == Location.BOUNDARY) return true;
    // JTS:     return false;
    // JTS:   }
    pub fn is_boundary_node(&self, geom_index: usize, coord: Coordinate<F>) -> bool {
        self.nodes
            .find(coord)
            .and_then(|node| node.label().on_position(geom_index))
            .map(|position| position == CoordPos::OnBoundary)
            .unwrap_or(false)
    }

    // JTS:   protected void insertEdge(Edge e)
    // JTS:   {
    // JTS:     edges.add(e);
    // JTS:   }
    pub fn insert_edge(&mut self, edge: Edge<F>) {
        self.edges.push(Rc::new(RefCell::new(edge)));
    }

    // JTS:   public void add(EdgeEnd e)
    // JTS:   {
    // JTS:     nodes.add(e);
    // JTS:     edgeEndList.add(e);
    // JTS:   }
    // JTS:
    // JTS:   public Iterator getNodeIterator() { return nodes.iterator(); }
    // JTS:   public Collection getNodes() { return nodes.values(); }
    // JTS:   public Node addNode(Node node) { return nodes.addNode(node); }
    // JTS:   public Node addNode(Coordinate coord) { return nodes.addNode(coord); }
    pub fn add_node_with_coordinate(&mut self, coord: Coordinate<F>) -> &mut CoordNode<F> {
        self.nodes.insert_node_with_coordinate(coord)
    }

    pub fn boundary_nodes(&self, geom_index: usize) -> impl Iterator<Item = &CoordNode<F>> {
        self.nodes.iter().filter(move |node| {
            matches!(
                node.label().on_position(geom_index),
                Some(CoordPos::OnBoundary)
            )
        })
    }

    // JTS:   /**
    // JTS:    * @return the node if found; null otherwise
    // JTS:    */
    // JTS:   public Node find(Coordinate coord) { return nodes.find(coord); }
    // JTS:
    // JTS:   /**
    // JTS:    * Add a set of edges to the graph.  For each edge two DirectedEdges
    // JTS:    * will be created.  DirectedEdges are NOT linked by this method.
    // JTS:    */
    // JTS:   public void addEdges(List edgesToAdd)
    // JTS:   {
    // JTS:     // create all the nodes for the edges
    // JTS:     for (Iterator it = edgesToAdd.iterator(); it.hasNext(); ) {
    // JTS:       Edge e = (Edge) it.next();
    // JTS:       edges.add(e);
    // JTS:
    // JTS:       DirectedEdge de1 = new DirectedEdge(e, true);
    // JTS:       DirectedEdge de2 = new DirectedEdge(e, false);
    // JTS:       de1.setSym(de2);
    // JTS:       de2.setSym(de1);
    // JTS:
    // JTS:       add(de1);
    // JTS:       add(de2);
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Link the DirectedEdges at the nodes of the graph.
    // JTS:    * This allows clients to link only a subset of nodes in the graph, for
    // JTS:    * efficiency (because they know that only a subset is of interest).
    // JTS:    */
    // JTS:   public void linkResultDirectedEdges()
    // JTS:   {
    // JTS:     for (Iterator nodeit = nodes.iterator(); nodeit.hasNext(); ) {
    // JTS:       Node node = (Node) nodeit.next();
    // JTS:       ((DirectedEdgeStar) node.getEdges()).linkResultDirectedEdges();
    // JTS:     }
    // JTS:   }
    // JTS:   /**
    // JTS:    * Link the DirectedEdges at the nodes of the graph.
    // JTS:    * This allows clients to link only a subset of nodes in the graph, for
    // JTS:    * efficiency (because they know that only a subset is of interest).
    // JTS:    */
    // JTS:   public void linkAllDirectedEdges()
    // JTS:   {
    // JTS:     for (Iterator nodeit = nodes.iterator(); nodeit.hasNext(); ) {
    // JTS:       Node node = (Node) nodeit.next();
    // JTS:       ((DirectedEdgeStar) node.getEdges()).linkAllDirectedEdges();
    // JTS:     }
    // JTS:   }
    // JTS:   /**
    // JTS:    * Returns the EdgeEnd which has edge e as its base edge
    // JTS:    * (MD 18 Feb 2002 - this should return a pair of edges)
    // JTS:    *
    // JTS:    * @return the edge, if found
    // JTS:    *    <code>null</code> if the edge was not found
    // JTS:    */
    // JTS:   public EdgeEnd findEdgeEnd(Edge e)
    // JTS:   {
    // JTS:     for (Iterator i = getEdgeEnds().iterator(); i.hasNext(); ) {
    // JTS:       EdgeEnd ee = (EdgeEnd) i.next();
    // JTS:       if (ee.getEdge() == e)
    // JTS:         return ee;
    // JTS:     }
    // JTS:     return null;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Returns the edge whose first two coordinates are p0 and p1
    // JTS:    *
    // JTS:    * @return the edge, if found
    // JTS:    *    <code>null</code> if the edge was not found
    // JTS:    */
    // JTS:   public Edge findEdge(Coordinate p0, Coordinate p1)
    // JTS:   {
    // JTS:     for (int i = 0; i < edges.size(); i++) {
    // JTS:       Edge e = (Edge) edges.get(i);
    // JTS:       Coordinate[] eCoord = e.getCoordinates();
    // JTS:       if (p0.equals(eCoord[0]) && p1.equals(eCoord[1]) )
    // JTS:         return e;
    // JTS:     }
    // JTS:     return null;
    // JTS:   }
    // JTS:   /**
    // JTS:    * Returns the edge which starts at p0 and whose first segment is
    // JTS:    * parallel to p1
    // JTS:    *
    // JTS:    * @return the edge, if found
    // JTS:    *    <code>null</code> if the edge was not found
    // JTS:    */
    // JTS:   public Edge findEdgeInSameDirection(Coordinate p0, Coordinate p1)
    // JTS:   {
    // JTS:     for (int i = 0; i < edges.size(); i++) {
    // JTS:       Edge e = (Edge) edges.get(i);
    // JTS:
    // JTS:       Coordinate[] eCoord = e.getCoordinates();
    // JTS:       if (matchInSameDirection(p0, p1, eCoord[0], eCoord[1]) )
    // JTS:         return e;
    // JTS:
    // JTS:       if (matchInSameDirection(p0, p1, eCoord[eCoord.length - 1], eCoord[eCoord.length - 2]) )
    // JTS:         return e;
    // JTS:     }
    // JTS:     return null;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * The coordinate pairs match if they define line segments lying in the same direction.
    // JTS:    * E.g. the segments are parallel and in the same quadrant
    // JTS:    * (as opposed to parallel and opposite!).
    // JTS:    */
    // JTS:   private boolean matchInSameDirection(Coordinate p0, Coordinate p1, Coordinate ep0, Coordinate ep1)
    // JTS:   {
    // JTS:     if (! p0.equals(ep0))
    // JTS:       return false;
    // JTS:
    // JTS:     if (Orientation.index(p0, p1, ep1) == Orientation.COLLINEAR
    // JTS:          && Quadrant.quadrant(p0, p1) == Quadrant.quadrant(ep0, ep1) )
    // JTS:       return true;
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   public void printEdges(PrintStream out)
    // JTS:   {
    // JTS:     out.println("Edges:");
    // JTS:     for (int i = 0; i < edges.size(); i++) {
    // JTS:       out.println("edge " + i + ":");
    // JTS:       Edge e = (Edge) edges.get(i);
    // JTS:       e.print(out);
    // JTS:       e.eiList.print(out);
    // JTS:     }
    // JTS:   }
    // JTS:   void debugPrint(Object o)
    // JTS:   {
    // JTS:     System.out.print(o);
    // JTS:   }
    // JTS:   void debugPrintln(Object o)
    // JTS:   {
    // JTS:     System.out.println(o);
    // JTS:   }
    // JTS:
    // JTS: }
}
