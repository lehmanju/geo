use crate::{
    kernels::HasKernel,
    line_intersection::{line_intersection, LineIntersection},
};
use aatree::AATreeMap;
use core::fmt::{self, Debug};
use geo_types::{CoordFloat, Line};
use std::{cmp::Ordering, collections::HashMap, fmt::Display};

struct StoppingPoint<'a, S, T> {
    /// All line start points at current x.
    start_events: Vec<&'a S>,
    /// All line end points at current x.
    end_events: Vec<&'a S>,
    /// All intersection points at current x.
    intersection_events: HashMap<*const S, (&'a S, T)>,
}

#[derive(Debug)]
struct LineStatus<'a, S, T: Debug + CoordFloat> {
    /// Currently active segment
    segment: &'a S,
    /// Corresponding intersection point of neighbor segment with greater y value
    intersection_point: Option<(Float<T>, &'a S)>,
}

struct Status<'a, S, T: Debug + CoordFloat> {
    segments: Vec<LineStatus<'a, S, T>>,
}

impl<T> StoppingPoint<'_, Line<T>, T>
where
    T: CoordFloat,
{
    fn new() -> Self {
        Self {
            start_events: Vec::new(),
            end_events: Vec::new(),
            intersection_events: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Float<T>(T)
where
    T: CoordFloat;

impl<T: CoordFloat + Display> Float<T> {
    pub fn new(from: T) -> Self {
        if !from.is_finite() {
            panic!("Invalid floating point value: {}", from);
        }
        Self(from)
    }
}

impl<T: CoordFloat + Display> From<T> for Float<T> {
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<T: CoordFloat> Eq for Float<T> {}

impl<T: CoordFloat> Ord for Float<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.partial_cmp(&rhs.0).unwrap()
    }
}

impl<T: CoordFloat> Debug for Float<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as Debug>::fmt(&self.0, f)
    }
}

impl<T: CoordFloat + Display> Display for Float<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as Display>::fmt(&self.0, f)
    }
}

pub fn sweepline<'a, I, F, T>(segments: I) -> Vec<(&'a Line<T>, &'a Line<T>, T)>
where
    I: IntoIterator<Item = &'a Line<T>>,
    T: CoordFloat + Display + 'a + HasKernel,
{
    // collect all start/end events
    let mut stopping = AATreeMap::<Float<T>, StoppingPoint<Line<T>, T>>::new();
    for s in segments {
        let start_x = Float::new(s.start.x);
        if let Some(e) = stopping.get_mut(&start_x) {
            e.start_events.push(s);
        } else {
            let mut stopping_point = StoppingPoint::new();
            stopping_point.start_events.push(s);
            stopping.insert(start_x, stopping_point);
        }

        let end_x = Float::new(s.end.x);
        if let Some(e) = stopping.get_mut(&end_x) {
            e.end_events.push(s);
        } else {
            let mut stopping_point = StoppingPoint::new();
            stopping_point.start_events.push(s);
            stopping.insert(end_x, stopping_point);
        }
    }

    let mut status_list = AATreeMap::<Float<T>, Status<'a, Line<T>, T>>::new();
    while let Some((x, point)) = stopping.pop_smallest() {
        // 1. process segment start events
        for start_event in point.start_events {
            let y = start_event.start.y;
            let y_pos = Float::new(y);
            // smaller neighbor of current segment
            let upper_neighbor = status_list.largest_leq_than_mut(&y_pos);
            let mut status: LineStatus<Line<T>, T> = LineStatus {
                segment: start_event,
                intersection_point: None,
            };
            // compute intersection with upper segments
            if let Some((_, upper_status)) = upper_neighbor {
                for seg in &mut upper_status.segments {
                    let line_intersection =
                        line_intersection(seg.segment.clone(), start_event.clone());
                    // if upper value has intersection points with other segments, replace it with this segment
                    // retrieve x value of upper's intersection point
                    if let Some((upper_x, upper_line)) = seg.intersection_point {
                        // retrieve stopping point at upper_x
                        let upper_intersection_point = stopping.get_mut(&upper_x).expect("Implementation error, expected reference to corresponding intersection point");
                        match line_intersection {
                            // if upper intersects with current segment
                            Some(intersect) => {
                                // follow link from upper_line to correct intersection point entry (multiple intersection points from different segments could be present at upper_x)
                                let (other_segment, intersection_y) = upper_intersection_point
                                .intersection_events
                                .get_mut(&(upper_line as *const Line<T>))
                                .expect(
                                    "Implementation error, expected intersection point in hash map",
                                );
                                // replace value of intersection point
                                match intersect {
                                    LineIntersection::SinglePoint {
                                        intersection: coord,
                                        ..
                                    } => {
                                        *other_segment = start_event;
                                        *intersection_y = coord.y;
                                    }
                                    LineIntersection::Collinear { .. } => {
                                        // segments intersect parallel, but don't cross
                                        // equals no intersection
                                        seg.intersection_point = None;
                                        upper_intersection_point
                                            .intersection_events
                                            .remove(&(upper_line as *const Line<T>));
                                    }
                                }
                            }
                            // if upper doesn't intersect with current segment, remove intersection
                            None => {
                                seg.intersection_point = None;
                                upper_intersection_point
                                    .intersection_events
                                    .remove(&(upper_line as *const Line<T>));
                            }
                        }
                    } else if let Some(intersect) = line_intersection {
                        // insert directly if lines intersect and no previous intersection point is present
                        if let LineIntersection::SinglePoint {
                            intersection: coord,
                            ..
                        } = intersect
                        {
                            seg.intersection_point = Some((Float::new(coord.x), start_event));
                            let mut stopping_new: StoppingPoint<Line<T>, T> = StoppingPoint::new();
                            stopping_new
                                .intersection_events
                                .insert(seg.segment, (start_event, coord.y));
                            stopping.insert(Float::new(coord.x), stopping_new);
                        }
                    }
                }
            }

            let lower_neighbor = status_list.smallest_geq_than_mut(&y_pos);
            // because the upper segment always contains the reference to its intersection point with the lower segment, we don't have to check for an existing intersection point
            // the current segment is new and inserted and doesn't have any entries yet
            if let Some((_, lower_status)) = lower_neighbor {
                for segment_status in lower_status.segments {
                    let line_intersection =
                        line_intersection(segment_status.segment.clone(), start_event.clone());
                    if let Some(intersect) = line_intersection {
                        if let LineIntersection::SinglePoint {
                            intersection: coord,
                            ..
                        } = intersect
                        {
                            segment_status.intersection_point =
                                Some((Float::new(coord.x), start_event));
                            let mut stopping_new: StoppingPoint<Line<T>, T> = StoppingPoint::new();
                            stopping_new
                                .intersection_events
                                .insert(segment_status.segment, (start_event, coord.y));
                            stopping.insert(Float::new(coord.x), stopping_new);
                            status.intersection_point =
                                Some((Float::new(coord.x), segment_status.segment));
                        }
                    }
                }
            }

            // insert current line status into status list

            if let Some(status_in_list) = status_list.get_mut(&y_pos) {
                status_in_list.segments.push(status);
            } else {
                let new_status = Status {
                    segments: vec![status],
                };
                status_list.insert(y_pos, new_status);
            }
        }
        // 2. process segment end events
        for end_event in point.end_events {
            let y_pos = Float::new(end_event.end.y);
            // query smaller and larger element
            let lower_neighbors = status_list
                .smallest_geq_than(&y_pos)
                .map(|(key, _)| key.clone());
            let upper_neighbors = status_list
                .largest_leq_than(&y_pos)
                .map(|(key, _)| key.clone());
            // remove element at y_pos
            
            if let Some(upper_neighbors) = upper_neighbors {
                for line_status in upper_neighbors {
                    line_status.intersection_point = None;
                    if let Some(lower) = lower_neighbors {
                        for lower_line_status in lower {
                            let line_intersection =
                                line_intersection(lower_line_status.segment.clone(), line_status.segment.clone());
                            if let Some(intersect) = line_intersection {
                                if let LineIntersection::SinglePoint {
                                    intersection: coord,
                                    ..
                                } = intersect
                                {
                                    let mut stopping_new: StoppingPoint<Line<T>, T> =
                                        StoppingPoint::new();
                                    stopping_new
                                        .intersection_events
                                        .insert(lower_line_status.segment, (line_status.segment, coord.y));
                                    stopping.insert(Float::new(coord.x), stopping_new);
                                    line_status.intersection_point =
                                        Some((Float::new(coord.x), lower_line_status.segment));
                                }
                            }
                        }
                    }
                }
            }
            //status.remove()

            // no intersection points with current segment should be left
            // remove self from status
        }
        for intersection_event in point.intersection_events {}
    }

    unimplemented!()
}
