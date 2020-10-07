// JTS: import org.locationtech.jts.geom.Location;

use super::{CoordPos, Direction};

use std::fmt;

// JTS:
// JTS: /**
// JTS:   * A TopologyLocation is the labelling of a
// JTS:   * GraphComponent's topological relationship to a single Geometry.
// JTS:   * <p>
// JTS:   * If the parent component is an area edge, each side and the edge itself
// JTS:   * have a topological location.  These locations are named
// JTS:   * <ul>
// JTS:   * <li> ON: on the edge
// JTS:   * <li> LEFT: left-hand side of the edge
// JTS:   * <li> RIGHT: right-hand side
// JTS:   * </ul>
// JTS:   * If the parent component is a line edge or node, there is a single
// JTS:   * topological relationship attribute, ON.
// JTS:   * <p>
// JTS:   * The possible values of a topological location are
// JTS:   * {Location.NONE, Location.EXTERIOR, Location.BOUNDARY, Location.INTERIOR}
// JTS:   * <p>
// JTS:   * The labelling is stored in an array location[j] where
// JTS:   * where j has the values ON, LEFT, RIGHT
// JTS:   * @version 1.7
// JTS:  */
// JTS: public class TopologyLocation {

/// A `TopologyPosition` is the labelling of a graph component's topological relationship to a
/// single Geometry for each of the component's [`Direction`s](Direction).
///
/// If the graph component is an _area_ edge, there is a position for each [`Direction`]:
/// - [`On`](Direction::On): on the edge
/// - [`Left`](Direction::Left): left-hand side of the edge
/// - [`Right`](Direction::Right): right-hand side
///
/// If the parent component is a _line_ edge or a node (a point), there is a single
/// topological relationship attribute for the [`On`](Direction::On) position.
///
/// See [`CoordPos`] for the possible values.
#[derive(Clone)]
pub(crate) enum TopologyPosition {
    Area {
        on: Option<CoordPos>,
        left: Option<CoordPos>,
        right: Option<CoordPos>,
    },
    LineOrPoint {
        on: Option<CoordPos>,
    },
}

impl fmt::Debug for TopologyPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_position(position: &Option<CoordPos>, f: &mut fmt::Formatter) -> fmt::Result {
            match position {
                Some(CoordPos::Inside) => write!(f, "i"),
                Some(CoordPos::OnBoundary) => write!(f, "b"),
                Some(CoordPos::Outside) => write!(f, "e"),
                None => write!(f, "_"),
            }
        }
        match self {
            Self::LineOrPoint { on } => fmt_position(on, f)?,
            Self::Area { on, left, right } => {
                fmt_position(left, f)?;
                fmt_position(on, f)?;
                fmt_position(right, f)?;
            }
        }
        Ok(())
    }
}

impl TopologyPosition {
    // JTS:
    // JTS:   int location[];
    // JTS:
    // JTS:   public TopologyLocation(int[] location)
    // JTS:   {
    // JTS:     init(location.length);
    // JTS:   }
    // JTS:   /**
    // JTS:    * Constructs a TopologyLocation specifying how points on, to the left of, and to the
    // JTS:    * right of some GraphComponent relate to some Geometry. Possible values for the
    // JTS:    * parameters are Location.NULL, Location.EXTERIOR, Location.BOUNDARY,
    // JTS:    * and Location.INTERIOR.
    // JTS:    * @see Location
    // JTS:    */
    // JTS:   public TopologyLocation(int on, int left, int right) {
    // JTS:    init(3);
    // JTS:    location[Position.ON] = on;
    // JTS:    location[Position.LEFT] = left;
    // JTS:    location[Position.RIGHT] = right;
    // JTS:   }
    pub fn area(on: CoordPos, left: CoordPos, right: CoordPos) -> Self {
        Self::Area {
            on: Some(on),
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn empty_area() -> Self {
        Self::Area {
            on: None,
            left: None,
            right: None,
        }
    }

    // JTS:   public TopologyLocation(int on) {
    // JTS:    init(1);
    // JTS:    location[Position.ON] = on;
    // JTS:   }
    pub fn line_or_point(on: CoordPos) -> Self {
        Self::LineOrPoint { on: Some(on) }
    }

    pub fn empty_line_or_point() -> Self {
        Self::LineOrPoint { on: None }
    }

    // JTS:   public TopologyLocation(TopologyLocation gl) {
    // JTS:     init(gl.location.length);
    // JTS:     if (gl != null) {
    // JTS:       for (int i = 0; i < location.length; i++) {
    // JTS:         location[i] = gl.location[i];
    // JTS:       }
    // JTS:     }
    // JTS:   }
    // JTS:   private void init(int size)
    // JTS:   {
    // JTS:     location = new int[size];
    // JTS:     setAllLocations(Location.NONE);
    // JTS:   }
    // JTS:   public int get(int posIndex)
    // JTS:   {
    // JTS:     if (posIndex < location.length) return location[posIndex];
    // JTS:     return Location.NONE;
    // JTS:   }
    pub fn get(&self, direction: Direction) -> Option<CoordPos> {
        match (direction, self) {
            (Direction::Left, Self::Area { left, .. }) => *left,
            (Direction::Right, Self::Area { right, .. }) => *right,
            (Direction::On, Self::LineOrPoint { on }) | (Direction::On, Self::Area { on, .. }) => {
                *on
            }
            (_, Self::LineOrPoint { .. }) => {
                panic!("LineOrPoint only has a position for `Direction::On`")
            }
        }
    }

    // JTS:   /**
    // JTS:    * @return true if all locations are NULL
    // JTS:    */
    // JTS:   public boolean isNull()
    // JTS:   {
    // JTS:     for (int i = 0; i < location.length; i++) {
    // JTS:       if (location[i] != Location.NONE) return false;
    // JTS:     }
    // JTS:     return true;
    // JTS:   }
    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            Self::LineOrPoint { on: None }
                | Self::Area {
                    on: None,
                    left: None,
                    right: None,
                }
        )
    }

    // JTS:   /**
    // JTS:    * @return true if any locations are NULL
    // JTS:    */
    // JTS:   public boolean isAnyNull()
    // JTS:   {
    // JTS:     for (int i = 0; i < location.length; i++) {
    // JTS:       if (location[i] == Location.NONE) return true;
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    pub fn is_any_empty(&self) -> bool {
        !matches!(
            self,
            Self::LineOrPoint { on: Some(_) }
                | Self::Area {
                    on: Some(_),
                    left: Some(_),
                    right: Some(_),
                }
        )
    }

    // JTS:   public boolean isEqualOnSide(TopologyLocation le, int locIndex)
    // JTS:   {
    // JTS:     return location[locIndex] == le.location[locIndex];
    // JTS:   }
    // JTS:   public boolean isArea() { return location.length > 1; }
    pub fn is_area(&self) -> bool {
        matches!(self, Self::Area { .. })
    }

    // JTS:   public boolean isLine() { return location.length == 1; }
    pub fn is_line(&self) -> bool {
        matches!(self, Self::LineOrPoint { .. })
    }

    // JTS:   public void flip()
    // JTS:   {
    // JTS:     if (location.length <= 1) return;
    // JTS:     int temp = location[Position.LEFT];
    // JTS:     location[Position.LEFT] = location[Position.RIGHT];
    // JTS:     location[Position.RIGHT] = temp;
    // JTS:   }
    pub fn flip(&mut self) {
        match self {
            Self::LineOrPoint { .. } => {}
            Self::Area { left, right, .. } => {
                std::mem::swap(left, right);
            }
        }
    }

    // JTS:   public void setAllLocations(int locValue)
    // JTS:   {
    // JTS:     for (int i = 0; i < location.length; i++) {
    // JTS:       location[i]     = locValue;
    // JTS:     }
    // JTS:   }
    pub fn set_all_positions(&mut self, position: CoordPos) {
        match self {
            Self::LineOrPoint { on } => {
                *on = Some(position);
            }
            Self::Area { on, left, right } => {
                *on = Some(position);
                *left = Some(position);
                *right = Some(position);
            }
        }
    }

    // JTS:   public void setAllLocationsIfNull(int locValue)
    // JTS:   {
    // JTS:     for (int i = 0; i < location.length; i++) {
    // JTS:       if (location[i] == Location.NONE) location[i]     = locValue;
    // JTS:     }
    // JTS:   }
    pub fn set_all_positions_if_empty(&mut self, position: CoordPos) {
        match self {
            Self::LineOrPoint { on } => {
                if on.is_none() {
                    *on = Some(position);
                }
            }
            Self::Area { on, left, right } => {
                if on.is_none() {
                    *on = Some(position);
                }
                if left.is_none() {
                    *left = Some(position);
                }
                if right.is_none() {
                    *right = Some(position);
                }
            }
        }
    }

    // JTS:   public void setLocation(int locIndex, int locValue)
    // JTS:   {
    // JTS:       location[locIndex] = locValue;
    // JTS:   }
    pub fn set_position(&mut self, direction: Direction, position: CoordPos) {
        match (direction, self) {
            (Direction::On, Self::LineOrPoint { on }) => *on = Some(position),
            (_, Self::LineOrPoint { .. }) => {
                panic!("invalid assignment dimensions for Self::Line")
            }
            (Direction::On, Self::Area { on, .. }) => *on = Some(position),
            (Direction::Left, Self::Area { left, .. }) => *left = Some(position),
            (Direction::Right, Self::Area { right, .. }) => *right = Some(position),
        }
    }

    // JTS:   public void setLocation(int locValue)
    // JTS:   {
    // JTS:     setLocation(Position.ON, locValue);
    // JTS:   }
    pub fn set_on_position(&mut self, position: CoordPos) {
        match self {
            Self::LineOrPoint { on } | Self::Area { on, .. } => {
                *on = Some(position);
            }
        }
    }

    // JTS:   public int[] getLocations() { return location; }
    // JTS:   public void setLocations(int on, int left, int right) {
    // JTS:       location[Position.ON] = on;
    // JTS:       location[Position.LEFT] = left;
    // JTS:       location[Position.RIGHT] = right;
    // JTS:   }
    pub fn set_locations(&mut self, new_on: CoordPos, new_left: CoordPos, new_right: CoordPos) {
        match self {
            Self::LineOrPoint { .. } => {
                error!("invalid assignment dimensions for {:?}", self);
                debug_assert!(false, "invalid assignment dimensions for {:?}", self);
            }
            Self::Area { on, left, right } => {
                *on = Some(new_on);
                *left = Some(new_left);
                *right = Some(new_right);
            }
        }
    }

    // JTS:   public boolean allPositionsEqual(int loc)
    // JTS:   {
    // JTS:     for (int i = 0; i < location.length; i++) {
    // JTS:       if (location[i] != loc) return false;
    // JTS:     }
    // JTS:     return true;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * merge updates only the NULL attributes of this object
    // JTS:    * with the attributes of another.
    // JTS:    */
    // JTS:   public void merge(TopologyLocation gl)
    // JTS:   {
    // JTS:     // if the src is an Area label & and the dest is not, increase the dest to be an Area
    // JTS:     if (gl.location.length > location.length) {
    // JTS:       int [] newLoc = new int[3];
    // JTS:       newLoc[Position.ON] = location[Position.ON];
    // JTS:       newLoc[Position.LEFT] = Location.NONE;
    // JTS:       newLoc[Position.RIGHT] = Location.NONE;
    // JTS:       location = newLoc;
    // JTS:     }
    // JTS:     for (int i = 0; i < location.length; i++) {
    // JTS:       if (location[i] == Location.NONE && i < gl.location.length)
    // JTS:         location[i] = gl.location[i];
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   public String toString()
    // JTS:   {
    // JTS:     StringBuffer buf = new StringBuffer();
    // JTS:     if (location.length > 1) buf.append(Location.toLocationSymbol(location[Position.LEFT]));
    // JTS:     buf.append(Location.toLocationSymbol(location[Position.ON]));
    // JTS:     if (location.length > 1) buf.append(Location.toLocationSymbol(location[Position.RIGHT]));
    // JTS:     return buf.toString();
    // JTS:   }
    // JTS: }
}
