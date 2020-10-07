use super::geomgraph::{Edge, EdgeEnd, EdgeIntersection};
use crate::GeoFloat;

use std::cell::RefCell;
use std::rc::Rc;

// JTS: /**
// JTS:  * An EdgeEndBuilder creates EdgeEnds for all the "split edges"
// JTS:  * created by the
// JTS:  * intersections determined for an Edge.
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: import java.util.ArrayList;
// JTS: import java.util.Iterator;
// JTS: import java.util.List;
// JTS:
// JTS: import org.locationtech.jts.geom.Coordinate;
// JTS: import org.locationtech.jts.geomgraph.Edge;
// JTS: import org.locationtech.jts.geomgraph.EdgeEnd;
// JTS: import org.locationtech.jts.geomgraph.EdgeIntersection;
// JTS: import org.locationtech.jts.geomgraph.EdgeIntersectionList;
// JTS: import org.locationtech.jts.geomgraph.Label;
// JTS:
// JTS: /**
// JTS:  * Computes the {@link EdgeEnd}s which arise from a noded {@link Edge}.
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
/// Computes the [`EdgeEnd`]s which arise from an [`Edge`] who has had it's `edge_intersections`
/// populated with self and proper [`EdgeIntersection`]s.
///
/// Based on [JTS's EdgeEndBuilder as of 1.18.1](https://github.com/locationtech/jts/blob/jts-1.18.1/modules/core/src/main/java/org/locationtech/jts/operation/relate/EdgeEndBuilder.java)
pub(crate) struct EdgeEndBuilder<F: GeoFloat> {
    _marker: std::marker::PhantomData<F>,
}

// JTS: public class EdgeEndBuilder {
// JTS:
// JTS:   public EdgeEndBuilder() {
// JTS:   }
impl<F: GeoFloat> EdgeEndBuilder<F> {
    pub fn new() -> Self {
        EdgeEndBuilder {
            _marker: std::marker::PhantomData,
        }
    }

    // JTS:   public List computeEdgeEnds(Iterator edges)
    // JTS:   {
    // JTS:     List l = new ArrayList();
    // JTS:     for (Iterator i = edges; i.hasNext(); ) {
    // JTS:       Edge e = (Edge) i.next();
    // JTS:       computeEdgeEnds(e, l);
    // JTS:     }
    // JTS:     return l;
    // JTS:   }
    pub fn compute_ends_for_edges(&self, edges: &[Rc<RefCell<Edge<F>>>]) -> Vec<EdgeEnd<F>> {
        let mut list = vec![];
        for edge in edges {
            self.compute_ends_for_edge(&mut edge.borrow_mut(), &mut list);
        }
        list
    }

    // JTS:   /**
    // JTS:    * Creates stub edges for all the intersections in this
    // JTS:    * Edge (if any) and inserts them into the graph.
    // JTS:    */
    // JTS:   public void computeEdgeEnds(Edge edge, List l)
    // JTS:   {
    /// Creates stub edges for all the intersections in the [`Edge`] (if any) and inserts them into
    /// the graph's `list`.
    fn compute_ends_for_edge(&self, edge: &mut Edge<F>, list: &mut Vec<EdgeEnd<F>>) {
        // JTS:     EdgeIntersectionList eiList = edge.getEdgeIntersectionList();
        // JTS: //Debug.print(eiList);
        // JTS:     // ensure that the list has entries for the first and last point of the edge
        // JTS:     eiList.addEndpoints();
        edge.add_edge_intersection_list_endpoints();

        // JTS:     Iterator it = eiList.iterator();
        // JTS:     EdgeIntersection eiPrev = null;
        // JTS:     EdgeIntersection eiCurr = null;
        // JTS:     // no intersections, so there is nothing to do
        // JTS:     if (! it.hasNext()) return;
        // JTS:     EdgeIntersection eiNext = (EdgeIntersection) it.next();

        let mut ei_iter = edge.edge_intersections().iter();
        let mut ei_prev;
        let mut ei_curr = None;
        let mut ei_next = ei_iter.next();
        if ei_next.is_none() {
            return;
        }

        // JTS:     do {
        loop {
            // JTS:       eiPrev = eiCurr;
            // JTS:       eiCurr = eiNext;
            // JTS:       eiNext = null;
            // JTS:       if (it.hasNext()) eiNext = (EdgeIntersection) it.next();
            ei_prev = ei_curr;
            ei_curr = ei_next;
            ei_next = ei_iter.next();
            // JTS:
            // JTS:       if (eiCurr != null) {
            // JTS:         createEdgeEndForPrev(edge, l, eiCurr, eiPrev);
            // JTS:         createEdgeEndForNext(edge, l, eiCurr, eiNext);
            // JTS:       }
            if let Some(ei_curr) = ei_curr {
                self.create_edge_end_for_prev(edge, list, ei_curr, ei_prev);
                self.create_edge_end_for_next(edge, list, ei_curr, ei_next);
            }

            // JTS:     } while (eiCurr != null);
            if ei_curr.is_none() {
                break;
            }
        }
        // JTS:
        // JTS:   }
    }

    // JTS:   /**
    // JTS:    * Create a EdgeStub for the edge before the intersection eiCurr.
    // JTS:    * The previous intersection is provided
    // JTS:    * in case it is the endpoint for the stub edge.
    // JTS:    * Otherwise, the previous point from the parent edge will be the endpoint.
    // JTS:    * <br>
    // JTS:    * eiCurr will always be an EdgeIntersection, but eiPrev may be null.
    // JTS:    */
    // JTS:   void createEdgeEndForPrev(
    // JTS:                       Edge edge,
    // JTS:                       List l,
    // JTS:                       EdgeIntersection eiCurr,
    // JTS:                       EdgeIntersection eiPrev)
    // JTS:   {
    /// Adds a `EdgeEnd`, if any, to `list` for the edge before the intersection ei_curr.
    ///
    /// The previous intersection is provided in case it is the endpoint for the stub edge.
    /// Otherwise, the previous point from the parent edge will be the endpoint.
    fn create_edge_end_for_prev(
        &self,
        edge: &Edge<F>,
        list: &mut Vec<EdgeEnd<F>>,
        ei_curr: &EdgeIntersection<F>,
        ei_prev: Option<&EdgeIntersection<F>>,
    ) {
        // JTS:     int iPrev = eiCurr.segmentIndex;
        // JTS:     if (eiCurr.dist == 0.0) {
        // JTS:       // if at the start of the edge there is no previous edge
        // JTS:       if (iPrev == 0) return;
        // JTS:       iPrev--;
        // JTS:     }
        let mut i_prev = ei_curr.segment_index();
        if ei_curr.distance().is_zero() {
            // if at the start of the edge there is no previous edge
            if i_prev == 0 {
                return;
            }
            i_prev -= 1;
        }

        // JTS:     Coordinate pPrev = edge.getCoordinate(iPrev);
        // JTS:     // if prev intersection is past the previous vertex, use it instead
        // JTS:     if (eiPrev != null && eiPrev.segmentIndex >= iPrev)
        // JTS:       pPrev = eiPrev.coord;
        // JTS:
        let mut coord_prev = edge.coords()[i_prev];
        // if prev intersection is past the previous vertex, use it instead
        if let Some(ei_prev) = ei_prev {
            if ei_prev.segment_index() >= i_prev {
                coord_prev = ei_prev.coordinate();
            }
        }

        // JTS:     Label label = new Label(edge.getLabel());
        let mut label = edge.label().clone();
        // JTS:     // since edgeStub is oriented opposite to it's parent edge, have to flip sides for edge label
        // JTS:     label.flip();
        // since edgeStub is oriented opposite to its parent edge, have to flip sides for edge label
        label.flip();

        // JTS:     EdgeEnd e = new EdgeEnd(edge, eiCurr.coord, pPrev, label);
        // JTS: //e.print(System.out);  System.out.println();
        // JTS:     l.add(e);
        // JTS:   }
        let edge_end = EdgeEnd::new(ei_curr.coordinate(), coord_prev, label);
        list.push(edge_end);
    }

    // JTS:     /**
    // JTS:      * Create a StubEdge for the edge after the intersection eiCurr.
    // JTS:      * The next intersection is provided
    // JTS:      * in case it is the endpoint for the stub edge.
    // JTS:      * Otherwise, the next point from the parent edge will be the endpoint.
    // JTS:      * <br>
    // JTS:      * eiCurr will always be an EdgeIntersection, but eiNext may be null.
    // JTS:      */
    // JTS:   void createEdgeEndForNext(
    // JTS:                       Edge edge,
    // JTS:                       List l,
    // JTS:                       EdgeIntersection eiCurr,
    // JTS:                       EdgeIntersection eiNext)
    // JTS:   {
    /// Adds a `EdgeEnd`, if any, to `list` for the edge after the intersection ei_curr.
    ///
    /// The next intersection is provided in case it is the endpoint for the stub edge.
    /// Otherwise, the next point from the parent edge will be the endpoint.
    fn create_edge_end_for_next(
        &self,
        edge: &Edge<F>,
        list: &mut Vec<EdgeEnd<F>>,
        ei_curr: &EdgeIntersection<F>,
        ei_next: Option<&EdgeIntersection<F>>,
    ) {
        // JTS:     int iNext = eiCurr.segmentIndex + 1;
        let i_next = ei_curr.segment_index() + 1;

        // JTS:     // if there is no next edge there is nothing to do
        // JTS:     if (iNext >= edge.getNumPoints() && eiNext == null) return;
        // if there is no next edge there is nothing to do
        if i_next >= edge.coords().len() && ei_next.is_none() {
            return;
        }

        // JTS:     Coordinate pNext = edge.getCoordinate(iNext);
        let mut coord_next = edge.coords()[i_next];

        // JTS:     // if the next intersection is in the same segment as the current, use it as the endpoint
        // JTS:     if (eiNext != null && eiNext.segmentIndex == eiCurr.segmentIndex)
        // JTS:       pNext = eiNext.coord;
        // if the next intersection is in the same segment as the current, use it as the endpoint
        if let Some(ei_next) = ei_next {
            if ei_next.segment_index() == ei_curr.segment_index() {
                coord_next = ei_next.coordinate();
            }
        }

        // JTS:     EdgeEnd e = new EdgeEnd(edge, eiCurr.coord, pNext, new Label(edge.getLabel()));
        // JTS: //Debug.println(e);
        // JTS:     l.add(e);
        let label = edge.label().clone();
        let edge_end = EdgeEnd::new(ei_curr.coordinate(), coord_next, label);
        list.push(edge_end);
        // JTS:   }
    }
    // JTS: }
}
