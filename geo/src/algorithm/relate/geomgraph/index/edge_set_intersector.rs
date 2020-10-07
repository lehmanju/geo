use super::super::Edge;
use super::SegmentIntersector;
use crate::{Coordinate, GeoFloat};

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) trait EdgeSetIntersector<F: GeoFloat> {
    // JTS: /**
    // JTS:  * Computes all self-intersections between edges in a set of edges,
    // JTS:  * allowing client to choose whether self-intersections are computed.
    // JTS:  *
    // JTS:  * @param edges a list of edges to test for intersections
    // JTS:  * @param si the SegmentIntersector to use
    // JTS:  * @param testAllSegments true if self-intersections are to be tested as well
    // JTS:  */
    // JTS: abstract public void computeIntersections(List edges, SegmentIntersector si, boolean testAllSegments);
    /// Compute all intersections between the edges within a set, recording those intersections on
    /// the intersecting edges.
    ///
    /// `edges`: the set of edges to check. Mutated to record any intersections.
    /// `check_for_self_intersecting_edges`: if false, an edge is not checked for intersections with itself.
    /// `segment_intersector`: the SegmentIntersector to use
    fn compute_intersections_within_set(
        &mut self,
        edges: &[Rc<RefCell<Edge<F>>>],
        check_for_self_intersecting_edges: bool,
        segment_intersector: &mut SegmentIntersector<F>,
    );

    // JTS: /**
    // JTS:   * Computes all mutual intersections between two sets of edges.
    // JTS:   */
    // JTS:  abstract public void computeIntersections(List edges0, List edges1, SegmentIntersector si);
    /// Compute all intersections between two sets of edges, recording those intersections on
    /// the intersecting edges.
    fn compute_intersections_between_sets(
        &mut self,
        edges0: &[Rc<RefCell<Edge<F>>>],
        edges1: &[Rc<RefCell<Edge<F>>>],
        segment_intersector: &mut SegmentIntersector<F>,
    );
}
