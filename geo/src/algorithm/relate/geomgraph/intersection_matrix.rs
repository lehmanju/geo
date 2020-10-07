use crate::algorithm::coordinate_position::CoordPos;
use crate::algorithm::dimensions::Dimensions;

// JTS: /**
// JTS:  * Models a <b>Dimensionally Extended Nine-Intersection Model (DE-9IM)</b> matrix.
// JTS:  * DE-9IM matrix values (such as "212FF1FF2")
// JTS:  * specify the topological relationship between two {@link Geometry}s.
// JTS:  * This class can also represent matrix patterns (such as "T*T******")
// JTS:  * which are used for matching instances of DE-9IM matrices.
// JTS:  * <p>
// JTS:  * DE-9IM matrices are 3x3 matrices with integer entries.
// JTS:  * The matrix indices {0,1,2} represent the topological locations
// JTS:  * that occur in a geometry (Interior, Boundary, Exterior).
// JTS:  * These are provided by the constants
// JTS:  * {@link Location#INTERIOR}, {@link Location#BOUNDARY}, and {@link Location#EXTERIOR}.
// JTS:  * <p>
// JTS:  * When used to specify the topological relationship between two geometries,
// JTS:  * the matrix entries represent the possible dimensions of each intersection:
// JTS:  * {@link Dimension#A} = 2, {@link Dimension#L} = 1, {@link Dimension#P} = 0 and {@link Dimension#FALSE} = -1.
// JTS:  * When used to represent a matrix pattern entries can have the additional values
// JTS:  * {@link Dimension#TRUE} {"T") and {@link Dimension#DONTCARE} ("*").
// JTS:  * <p>
// JTS:  * For a description of the DE-9IM and the spatial predicates derived from it,
// JTS:  * see the following references:
// JTS:  * <ul>
// JTS:  * <li><i><a href="http://www.opengis.org/techno/specs.htm">
// JTS:  * OGC 99-049 OpenGIS Simple Features Specification for SQL</a></i>
// JTS:  * , Section 2.1.13</li>
// JTS:  * <li><i><a href="http://portal.opengeospatial.org/files/?artifact_id=25355">
// JTS:  * OGC 06-103r4 OpenGIS Implementation Standard for Geographic information - Simple feature access - Part 1: Common architecture</a></i>
// JTS:  * , Section 6.1.15 (which provides some further details on certain predicate specifications).
// JTS:  * </li>
// JTS:  * <li>Wikipedia article on <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a></li>
// JTS:  * </ul>
// JTS:  * <p>
// JTS:  * Methods are provided to:
// JTS:  *  <UL>
// JTS:  *    <LI>set and query the elements of the matrix in a convenient fashion
// JTS:  *    <LI>convert to and from the standard string representation (specified in
// JTS:  *    SFS Section 2.1.13.2).
// JTS:  *    <LI>test if a matrix matches a given pattern string.
// JTS:  *    <li>test if a matrix (possibly with geometry dimensions) matches a standard named spatial predicate
// JTS:  *  </UL>
// JTS:  *
// JTS:  *@version 1.7
// JTS:  */
/// Models a *Dimensionally Extended Nine-Intersection Model (DE-9IM)* matrix.
///
/// DE-9IM matrix values (such as "212FF1FF2") specify the topological relationship between
/// two [Geometeries](struct.Geometry.html).
///
/// DE-9IM matrices are 3x3 matrices that represent the topological locations
/// that occur in a geometry (Interior, Boundary, Exterior).
///
/// The indices are provided by the enum cases
/// [CoordPos::Inside, CoordPos::OnBoundary, CoordPos::Outside](CoordPos).
///
/// The matrix entries represent the [Dimensions](enum.Dimension.html) of each intersection.
///
/// For a description of the DE-9IM and the spatial predicates derived from it,
/// see the following references:
/// - [OGC 99-049 OpenGIS Simple Features Specification for SQL](http://portal.opengeospatial.org/files/?artifact_id=829), Section 2.1.13
/// - [OGC 06-103r4 OpenGIS Implementation Standard for Geographic information - Simple feature access - Part 1: Common architecture](http://portal.opengeospatial.org/files/?artifact_id=25355), Section 6.1.15 (which provides some further details on certain predicate specifications).
/// - Wikipedia article on [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM)
///
/// This implementation is heavily based on that from the [JTS project](https://github.com/locationtech/jts/blob/master/modules/core/src/main/java/org/locationtech/jts/geom/IntersectionMatrix.java).
// JTS: public class IntersectionMatrix implements Cloneable {
#[derive(PartialEq, Eq)]
pub struct IntersectionMatrix(LocationArray<LocationArray<Dimensions>>);

#[derive(PartialEq, Eq, Clone, Copy)]
struct LocationArray<T>([T; 3]);
impl<T> LocationArray<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T> std::ops::Index<CoordPos> for LocationArray<T> {
    type Output = T;

    fn index(&self, index: CoordPos) -> &Self::Output {
        match index {
            CoordPos::Inside => &self.0[0],
            CoordPos::OnBoundary => &self.0[1],
            CoordPos::Outside => &self.0[2],
        }
    }
}

impl<T> std::ops::IndexMut<CoordPos> for LocationArray<T> {
    fn index_mut(&mut self, index: CoordPos) -> &mut Self::Output {
        match index {
            CoordPos::Inside => &mut self.0[0],
            CoordPos::OnBoundary => &mut self.0[1],
            CoordPos::Outside => &mut self.0[2],
        }
    }
}

#[derive(Debug)]
pub struct InvalidInput {
    message: String,
}

impl InvalidInput {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::error::Error for InvalidInput {}
impl std::fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid input:  {}", self.message)
    }
}

impl std::fmt::Debug for IntersectionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn char_for_dim(dim: &Dimensions) -> &'static str {
            match dim {
                Dimensions::Empty => "F",
                Dimensions::ZeroDimensional => "0",
                Dimensions::OneDimensional => "1",
                Dimensions::TwoDimensional => "2",
            }
        }
        let text = self
            .0
            .iter()
            .flat_map(|r| r.iter().map(char_for_dim))
            .collect::<Vec<&str>>()
            .join("");

        write!(f, "IntersectionMatrix({})", &text)
    }
}

impl IntersectionMatrix {
    // JTS:   /**
    // JTS:    *  Internal representation of this <code>IntersectionMatrix</code>.
    // JTS:    */
    // JTS:   private int[][] matrix;
    // JTS:
    // JTS:   /**
    // JTS:    *  Creates an <code>IntersectionMatrix</code> with <code>FALSE</code>
    // JTS:    *  dimension values.
    // JTS:    */
    // JTS:   public IntersectionMatrix() {
    // JTS:     matrix = new int[3][3];
    // JTS:     setAll(Dimension.FALSE);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Creates an <code>IntersectionMatrix</code> with the given dimension
    // JTS:    *  symbols.
    // JTS:    *
    // JTS:    *@param  elements  a String of nine dimension symbols in row major order
    // JTS:    */
    // JTS:   public IntersectionMatrix(String elements) {
    // JTS:     this();
    // JTS:     set(elements);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Creates an <code>IntersectionMatrix</code> with the same elements as
    // JTS:    *  <code>other</code>.
    // JTS:    *
    // JTS:    *@param  other  an <code>IntersectionMatrix</code> to copy
    // JTS:    */
    // JTS:   public IntersectionMatrix(IntersectionMatrix other) {
    // JTS:     this();
    // JTS:     matrix[Location.INTERIOR][Location.INTERIOR] = other.matrix[Location.INTERIOR][Location.INTERIOR];
    // JTS:     matrix[Location.INTERIOR][Location.BOUNDARY] = other.matrix[Location.INTERIOR][Location.BOUNDARY];
    // JTS:     matrix[Location.INTERIOR][Location.EXTERIOR] = other.matrix[Location.INTERIOR][Location.EXTERIOR];
    // JTS:     matrix[Location.BOUNDARY][Location.INTERIOR] = other.matrix[Location.BOUNDARY][Location.INTERIOR];
    // JTS:     matrix[Location.BOUNDARY][Location.BOUNDARY] = other.matrix[Location.BOUNDARY][Location.BOUNDARY];
    // JTS:     matrix[Location.BOUNDARY][Location.EXTERIOR] = other.matrix[Location.BOUNDARY][Location.EXTERIOR];
    // JTS:     matrix[Location.EXTERIOR][Location.INTERIOR] = other.matrix[Location.EXTERIOR][Location.INTERIOR];
    // JTS:     matrix[Location.EXTERIOR][Location.BOUNDARY] = other.matrix[Location.EXTERIOR][Location.BOUNDARY];
    // JTS:     matrix[Location.EXTERIOR][Location.EXTERIOR] = other.matrix[Location.EXTERIOR][Location.EXTERIOR];
    // JTS:   }

    pub fn empty() -> Self {
        IntersectionMatrix(LocationArray([LocationArray([Dimensions::Empty; 3]); 3]))
    }

    // JTS:   /**
    // JTS:    * Adds one matrix to another.
    // JTS:    * Addition is defined by taking the maximum dimension value of each position
    // JTS:    * in the summand matrices.
    // JTS:    *
    // JTS:    * @param im the matrix to add
    // JTS:    */
    // JTS:   public void add(IntersectionMatrix im)
    // JTS:   {
    // JTS:     for (int i = 0; i < 3; i++) {
    // JTS:       for (int j = 0; j < 3; j++) {
    // JTS:         setAtLeast(i, j, im.get(i, j));
    // JTS:       }
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Tests if the dimension value matches <tt>TRUE</tt>
    // JTS:    *  (i.e.  has value 0, 1, 2 or TRUE).
    // JTS:    *
    // JTS:    *@param  actualDimensionValue     a number that can be stored in the <code>IntersectionMatrix</code>
    // JTS:    *      . Possible values are <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>.
    // JTS:    *@return true if the dimension value matches TRUE
    // JTS:    */
    // JTS:   public static boolean isTrue(int actualDimensionValue) {
    // JTS:     if (actualDimensionValue >= 0 || actualDimensionValue  == Dimension.TRUE) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Tests if the dimension value satisfies the dimension symbol.
    // JTS:    *
    // JTS:    *@param  actualDimensionValue     a number that can be stored in the <code>IntersectionMatrix</code>
    // JTS:    *      . Possible values are <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>.
    // JTS:    *@param  requiredDimensionSymbol  a character used in the string
    // JTS:    *      representation of an <code>IntersectionMatrix</code>. Possible values
    // JTS:    *      are <code>{T, F, * , 0, 1, 2}</code>.
    // JTS:    *@return                          true if the dimension symbol matches
    // JTS:    *      the dimension value
    // JTS:    */
    // JTS:   public static boolean matches(int actualDimensionValue, char requiredDimensionSymbol) {
    // JTS:     if (requiredDimensionSymbol == Dimension.SYM_DONTCARE) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     if (requiredDimensionSymbol == Dimension.SYM_TRUE && (actualDimensionValue >= 0 || actualDimensionValue
    // JTS:          == Dimension.TRUE)) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     if (requiredDimensionSymbol == Dimension.SYM_FALSE && actualDimensionValue == Dimension.FALSE) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     if (requiredDimensionSymbol == Dimension.SYM_P && actualDimensionValue == Dimension.P) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     if (requiredDimensionSymbol == Dimension.SYM_L && actualDimensionValue == Dimension.L) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     if (requiredDimensionSymbol == Dimension.SYM_A && actualDimensionValue == Dimension.A) {
    // JTS:       return true;
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Tests if each of the actual dimension symbols in a matrix string satisfies the
    // JTS:    *  corresponding required dimension symbol in a pattern string.
    // JTS:    *
    // JTS:    *@param  actualDimensionSymbols    nine dimension symbols to validate.
    // JTS:    *      Possible values are <code>{T, F, * , 0, 1, 2}</code>.
    // JTS:    *@param  requiredDimensionSymbols  nine dimension symbols to validate
    // JTS:    *      against. Possible values are <code>{T, F, * , 0, 1, 2}</code>.
    // JTS:    *@return                           true if each of the required dimension
    // JTS:    *      symbols encompass the corresponding actual dimension symbol
    // JTS:    */
    // JTS:   public static boolean matches(String actualDimensionSymbols, String requiredDimensionSymbols) {
    // JTS:     IntersectionMatrix m = new IntersectionMatrix(actualDimensionSymbols);
    // JTS:     return m.matches(requiredDimensionSymbols);
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Changes the value of one of this <code>IntersectionMatrix</code>s
    // JTS:    *  elements.
    // JTS:    *
    // JTS:    *@param  row             the row of this <code>IntersectionMatrix</code>,
    // JTS:    *      indicating the interior, boundary or exterior of the first <code>Geometry</code>
    // JTS:    *@param  column          the column of this <code>IntersectionMatrix</code>,
    // JTS:    *      indicating the interior, boundary or exterior of the second <code>Geometry</code>
    // JTS:    *@param  dimensionValue  the new value of the element
    // JTS:    */
    // JTS:   public void set(int row, int column, int dimensionValue) {
    // JTS:     matrix[row][column] = dimensionValue;
    // JTS:   }
    pub(crate) fn set(
        &mut self,
        position_a: CoordPos,
        position_b: CoordPos,
        dimensionality: Dimensions,
    ) {
        self.0[position_a][position_b] = dimensionality;
    }

    // JTS:   /**
    // JTS:    *  Changes the elements of this <code>IntersectionMatrix</code> to the
    // JTS:    *  dimension symbols in <code>dimensionSymbols</code>.
    // JTS:    *
    // JTS:    *@param  dimensionSymbols  nine dimension symbols to which to set this <code>IntersectionMatrix</code>
    // JTS:    *      s elements. Possible values are <code>{T, F, * , 0, 1, 2}</code>
    // JTS:    */
    // JTS:   public void set(String dimensionSymbols) {
    // JTS:     for (int i = 0; i < dimensionSymbols.length(); i++) {
    // JTS:       int row = i / 3;
    // JTS:       int col = i % 3;
    // JTS:       matrix[row][col] = Dimension.toDimensionValue(dimensionSymbols.charAt(i));
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Changes the specified element to <code>minimumDimensionValue</code> if the
    // JTS:    *  element is less.
    // JTS:    *
    // JTS:    *@param  row                    the row of this <code>IntersectionMatrix</code>
    // JTS:    *      , indicating the interior, boundary or exterior of the first <code>Geometry</code>
    // JTS:    *@param  column                 the column of this <code>IntersectionMatrix</code>
    // JTS:    *      , indicating the interior, boundary or exterior of the second <code>Geometry</code>
    // JTS:    *@param  minimumDimensionValue  the dimension value with which to compare the
    // JTS:    *      element. The order of dimension values from least to greatest is
    // JTS:    *      <code>{DONTCARE, TRUE, FALSE, 0, 1, 2}</code>.
    // JTS:    */
    // JTS:   public void setAtLeast(int row, int column, int minimumDimensionValue) {
    // JTS:     if (matrix[row][column] < minimumDimensionValue) {
    // JTS:       matrix[row][column] = minimumDimensionValue;
    // JTS:     }
    // JTS:   }
    pub(crate) fn set_at_least(
        &mut self,
        position_a: CoordPos,
        position_b: CoordPos,
        minimum_dimension_value: Dimensions,
    ) {
        if self.0[position_a][position_b] < minimum_dimension_value {
            self.0[position_a][position_b] = minimum_dimension_value;
        }
    }

    // JTS:   /**
    // JTS:    *  If row &gt;= 0 and column &gt;= 0, changes the specified element to <code>minimumDimensionValue</code>
    // JTS:    *  if the element is less. Does nothing if row &lt;0 or column &lt; 0.
    // JTS:    *
    // JTS:    *@param  row                    the row of this <code>IntersectionMatrix</code>
    // JTS:    *      , indicating the interior, boundary or exterior of the first <code>Geometry</code>
    // JTS:    *@param  column                 the column of this <code>IntersectionMatrix</code>
    // JTS:    *      , indicating the interior, boundary or exterior of the second <code>Geometry</code>
    // JTS:    *@param  minimumDimensionValue  the dimension value with which to compare the
    // JTS:    *      element. The order of dimension values from least to greatest is
    // JTS:    *      <code>{DONTCARE, TRUE, FALSE, 0, 1, 2}</code>.
    // JTS:    */
    // JTS:   public void setAtLeastIfValid(int row, int column, int minimumDimensionValue) {
    // JTS:     if (row >= 0 && column >= 0) {
    // JTS:       setAtLeast(row, column, minimumDimensionValue);
    // JTS:     }
    // JTS:   }
    pub(crate) fn set_at_least_if_valid(
        &mut self,
        position_a: Option<CoordPos>,
        position_b: Option<CoordPos>,
        minimum_dimension_value: Dimensions,
    ) {
        if let Some(position_a) = position_a {
            if let Some(position_b) = position_b {
                self.set_at_least(position_a, position_b, minimum_dimension_value);
            }
        }
    }

    // JTS:   /**
    // JTS:    *  For each element in this <code>IntersectionMatrix</code>, changes the
    // JTS:    *  element to the corresponding minimum dimension symbol if the element is
    // JTS:    *  less.
    // JTS:    *
    // JTS:    *@param  minimumDimensionSymbols  nine dimension symbols with which to
    // JTS:    *      compare the elements of this <code>IntersectionMatrix</code>. The
    // JTS:    *      order of dimension values from least to greatest is <code>{DONTCARE, TRUE, FALSE, 0, 1, 2}</code>
    // JTS:    *      .
    // JTS:    */
    // JTS:   public void setAtLeast(String minimumDimensionSymbols) {
    // JTS:     for (int i = 0; i < minimumDimensionSymbols.length(); i++) {
    // JTS:       int row = i / 3;
    // JTS:       int col = i % 3;
    // JTS:       setAtLeast(row, col, Dimension.toDimensionValue(minimumDimensionSymbols.charAt(i)));
    // JTS:     }
    // JTS:   }
    pub(crate) fn set_at_least_from_string(
        &mut self,
        dimensions: &str,
    ) -> Result<(), InvalidInput> {
        if dimensions.len() != 9 {
            let message = format!("Expected dimensions length 9, found: {}", dimensions.len());
            return Err(InvalidInput::new(message));
        }

        let mut chars = dimensions.chars();
        for a in &[CoordPos::Inside, CoordPos::OnBoundary, CoordPos::Outside] {
            for b in &[CoordPos::Inside, CoordPos::OnBoundary, CoordPos::Outside] {
                match chars.next().expect("already validated length is 9") {
                    '0' => self.0[*a][*b] = self.0[*a][*b].max(Dimensions::ZeroDimensional),
                    '1' => self.0[*a][*b] = self.0[*a][*b].max(Dimensions::OneDimensional),
                    '2' => self.0[*a][*b] = self.0[*a][*b].max(Dimensions::TwoDimensional),
                    'F' => {}
                    other => {
                        let message = format!("expected '0', '1', '2', or 'F'. Found: {}", other);
                        // We should make this return Err if this method becomes public
                        return Err(InvalidInput::new(message));
                    }
                }
            }
        }

        Ok(())
    }

    // JTS:
    // JTS:   /**
    // JTS:    *  Changes the elements of this <code>IntersectionMatrix</code> to <code>dimensionValue</code>
    // JTS:    *  .
    // JTS:    *
    // JTS:    *@param  dimensionValue  the dimension value to which to set this <code>IntersectionMatrix</code>
    // JTS:    *      s elements. Possible values <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>
    // JTS:    *      .
    // JTS:    */
    // JTS:   public void setAll(int dimensionValue) {
    // JTS:     for (int ai = 0; ai < 3; ai++) {
    // JTS:       for (int bi = 0; bi < 3; bi++) {
    // JTS:         matrix[ai][bi] = dimensionValue;
    // JTS:       }
    // JTS:     }
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Returns the value of one of this matrix
    // JTS:    *  entries.
    // JTS:    *  The value of the provided index is one of the
    // JTS:    *  values from the {@link Location} class.
    // JTS:    *  The value returned is a constant
    // JTS:    *  from the {@link Dimension} class.
    // JTS:    *
    // JTS:    *@param  row     the row of this <code>IntersectionMatrix</code>, indicating
    // JTS:    *      the interior, boundary or exterior of the first <code>Geometry</code>
    // JTS:    *@param  column  the column of this <code>IntersectionMatrix</code>,
    // JTS:    *      indicating the interior, boundary or exterior of the second <code>Geometry</code>
    // JTS:    *@return         the dimension value at the given matrix position.
    // JTS:    */
    // JTS:   public int get(int row, int column) {
    // JTS:     return matrix[row][column];
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Tests if this matrix matches <code>[FF*FF****]</code>.
    // JTS:    *
    // JTS:    *@return    <code>true</code> if the two <code>Geometry</code>s related by
    // JTS:    *      this matrix are disjoint
    // JTS:    */
    // JTS:   public boolean isDisjoint() {
    // JTS:     return
    // JTS:         matrix[Location.INTERIOR][Location.INTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.INTERIOR][Location.BOUNDARY] == Dimension.FALSE &&
    // JTS:         matrix[Location.BOUNDARY][Location.INTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.BOUNDARY][Location.BOUNDARY] == Dimension.FALSE;
    // JTS:   }
    /// Tests if this matrix matches `[FF*FF****]`.
    ///
    /// returns `true` if the two geometries related by this matrix are disjoint
    pub fn is_disjoint(&self) -> bool {
        self.0[CoordPos::Inside][CoordPos::Inside] == Dimensions::Empty
            && self.0[CoordPos::Inside][CoordPos::OnBoundary] == Dimensions::Empty
            && self.0[CoordPos::OnBoundary][CoordPos::Inside] == Dimensions::Empty
            && self.0[CoordPos::OnBoundary][CoordPos::OnBoundary] == Dimensions::Empty
    }

    // JTS:
    // JTS:   /**
    // JTS:    *  Tests if <code>isDisjoint</code> returns false.
    // JTS:    *
    // JTS:    *@return <code>true</code> if the two <code>Geometry</code>s related by
    // JTS:    *      this matrix intersect
    // JTS:    */
    // JTS:   public boolean isIntersects() {
    // JTS:     return ! isDisjoint();
    // JTS:   }
    /// Tests if `is_disjoint` returns false.
    ///
    /// returns `true` if the two geometries related by this matrix intersect.
    pub fn is_intersects(&self) -> bool {
        !self.is_disjoint()
    }

    // JTS:   /**
    // JTS:    *  Tests if this matrix matches
    // JTS:    *  <code>[FT*******]</code>, <code>[F**T*****]</code> or <code>[F***T****]</code>.
    // JTS:    *
    // JTS:    *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
    // JTS:    *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
    // JTS:    *@return                       <code>true</code> if the two <code>Geometry</code>
    // JTS:    *      s related by this matrix touch; Returns false
    // JTS:    *      if both <code>Geometry</code>s are points.
    // JTS:    */
    // JTS:   public boolean isTouches(int dimensionOfGeometryA, int dimensionOfGeometryB) {
    // JTS:     if (dimensionOfGeometryA > dimensionOfGeometryB) {
    // JTS:       //no need to get transpose because pattern matrix is symmetrical
    // JTS:       return isTouches(dimensionOfGeometryB, dimensionOfGeometryA);
    // JTS:     }
    // JTS:     if ((dimensionOfGeometryA == Dimension.A && dimensionOfGeometryB == Dimension.A) ||
    // JTS:         (dimensionOfGeometryA == Dimension.L && dimensionOfGeometryB == Dimension.L) ||
    // JTS:         (dimensionOfGeometryA == Dimension.L && dimensionOfGeometryB == Dimension.A) ||
    // JTS:         (dimensionOfGeometryA == Dimension.P && dimensionOfGeometryB == Dimension.A) ||
    // JTS:         (dimensionOfGeometryA == Dimension.P && dimensionOfGeometryB == Dimension.L)) {
    // JTS:       return matrix[Location.INTERIOR][Location.INTERIOR] == Dimension.FALSE &&
    // JTS:           (isTrue(matrix[Location.INTERIOR][Location.BOUNDARY])
    // JTS:            || isTrue(matrix[Location.BOUNDARY][Location.INTERIOR])
    // JTS:            || isTrue(matrix[Location.BOUNDARY][Location.BOUNDARY]));
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Tests whether this geometry crosses the
    // JTS:    * specified geometry.
    // JTS:    * <p>
    // JTS:    * The <code>crosses</code> predicate has the following equivalent definitions:
    // JTS:    * <ul>
    // JTS:    * <li>The geometries have some but not all interior points in common.
    // JTS:    * <li>The DE-9IM Intersection Matrix for the two geometries matches
    // JTS:    *   <ul>
    // JTS:    *    <li><code>[T*T******]</code> (for P/L, P/A, and L/A situations)
    // JTS:    *    <li><code>[T*****T**]</code> (for L/P, L/A, and A/L situations)
    // JTS:    *    <li><code>[0********]</code> (for L/L situations)
    // JTS:    *   </ul>
    // JTS:    * </ul>
    // JTS:    * For any other combination of dimensions this predicate returns <code>false</code>.
    // JTS:    * <p>
    // JTS:    * The SFS defined this predicate only for P/L, P/A, L/L, and L/A situations.
    // JTS:    * JTS extends the definition to apply to L/P, A/P and A/L situations as well.
    // JTS:    * This makes the relation symmetric.
    // JTS:    *
    // JTS:    *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
    // JTS:    *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
    // JTS:    *@return                       <code>true</code> if the two <code>Geometry</code>s
    // JTS:    *      related by this matrix cross.
    // JTS:    */
    // JTS:   public boolean isCrosses(int dimensionOfGeometryA, int dimensionOfGeometryB) {
    // JTS:     if ((dimensionOfGeometryA == Dimension.P && dimensionOfGeometryB == Dimension.L) ||
    // JTS:         (dimensionOfGeometryA == Dimension.P && dimensionOfGeometryB == Dimension.A) ||
    // JTS:         (dimensionOfGeometryA == Dimension.L && dimensionOfGeometryB == Dimension.A)) {
    // JTS:       return isTrue(matrix[Location.INTERIOR][Location.INTERIOR]) &&
    // JTS:       isTrue(matrix[Location.INTERIOR][Location.EXTERIOR]);
    // JTS:     }
    // JTS:     if ((dimensionOfGeometryA == Dimension.L && dimensionOfGeometryB == Dimension.P) ||
    // JTS:         (dimensionOfGeometryA == Dimension.A && dimensionOfGeometryB == Dimension.P) ||
    // JTS:         (dimensionOfGeometryA == Dimension.A && dimensionOfGeometryB == Dimension.L)) {
    // JTS:       return isTrue(matrix[Location.INTERIOR][Location.INTERIOR]) &&
    // JTS:       isTrue(matrix[Location.EXTERIOR][Location.INTERIOR]);
    // JTS:     }
    // JTS:     if (dimensionOfGeometryA == Dimension.L && dimensionOfGeometryB == Dimension.L) {
    // JTS:       return matrix[Location.INTERIOR][Location.INTERIOR] == 0;
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Tests whether this matrix matches <code>[T*F**F***]</code>.
    // JTS:    *
    // JTS:    *@return    <code>true</code> if the first <code>Geometry</code> is within
    // JTS:    *      the second
    // JTS:    */
    // JTS:   public boolean isWithin() {
    // JTS:     return isTrue(matrix[Location.INTERIOR][Location.INTERIOR]) &&
    // JTS:         matrix[Location.INTERIOR][Location.EXTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.BOUNDARY][Location.EXTERIOR] == Dimension.FALSE;
    // JTS:   }
    /// Tests whether this matrix matches `[T*F**F***]`.
    ///
    /// returns `true` if the first geometry is within the second.
    pub fn is_within(&self) -> bool {
        self.0[CoordPos::Inside][CoordPos::Inside] != Dimensions::Empty
            && self.0[CoordPos::Inside][CoordPos::Outside] == Dimensions::Empty
            && self.0[CoordPos::OnBoundary][CoordPos::Outside] == Dimensions::Empty
    }

    // JTS:   /**
    // JTS:    * Tests whether this matrix matches [T*****FF*[.
    // JTS:    *
    // JTS:    *@return    <code>true</code> if the first <code>Geometry</code> contains the
    // JTS:    *      second
    // JTS:    */
    // JTS:   public boolean isContains() {
    // JTS:     return isTrue(matrix[Location.INTERIOR][Location.INTERIOR]) &&
    // JTS:         matrix[Location.EXTERIOR][Location.INTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.EXTERIOR][Location.BOUNDARY] == Dimension.FALSE;
    // JTS:   }
    /// Tests whether this matrix matches `[T*****FF*]`.
    ///
    /// returns `true` if the first geometry contains the second.
    pub fn is_contains(&self) -> bool {
        self.0[CoordPos::Inside][CoordPos::Inside] != Dimensions::Empty
            && self.0[CoordPos::Outside][CoordPos::Inside] == Dimensions::Empty
            && self.0[CoordPos::Outside][CoordPos::OnBoundary] == Dimensions::Empty
    }

    // JTS:   /**
    // JTS:    * Tests if this matrix matches
    // JTS:    *    <code>[T*****FF*]</code>
    // JTS:    * or <code>[*T****FF*]</code>
    // JTS:    * or <code>[***T**FF*]</code>
    // JTS:    * or <code>[****T*FF*]</code>
    // JTS:    *
    // JTS:    *@return    <code>true</code> if the first <code>Geometry</code> covers the
    // JTS:    *      second
    // JTS:    */
    // JTS:   public boolean isCovers() {
    // JTS:     boolean hasPointInCommon =
    // JTS:         isTrue(matrix[Location.INTERIOR][Location.INTERIOR])
    // JTS:         || isTrue(matrix[Location.INTERIOR][Location.BOUNDARY])
    // JTS:         || isTrue(matrix[Location.BOUNDARY][Location.INTERIOR])
    // JTS:         || isTrue(matrix[Location.BOUNDARY][Location.BOUNDARY]);
    // JTS:
    // JTS:     return hasPointInCommon &&
    // JTS:         matrix[Location.EXTERIOR][Location.INTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.EXTERIOR][Location.BOUNDARY] == Dimension.FALSE;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *Tests if this matrix matches
    // JTS:    *    <code>[T*F**F***]</code>
    // JTS:    * or <code>[*TF**F***]</code>
    // JTS:    * or <code>[**FT*F***]</code>
    // JTS:    * or <code>[**F*TF***]</code>
    // JTS:    *
    // JTS:    *@return    <code>true</code> if the first <code>Geometry</code>
    // JTS:    * is covered by the second
    // JTS:    */
    // JTS:   public boolean isCoveredBy() {
    // JTS:     boolean hasPointInCommon =
    // JTS:         isTrue(matrix[Location.INTERIOR][Location.INTERIOR])
    // JTS:         || isTrue(matrix[Location.INTERIOR][Location.BOUNDARY])
    // JTS:         || isTrue(matrix[Location.BOUNDARY][Location.INTERIOR])
    // JTS:         || isTrue(matrix[Location.BOUNDARY][Location.BOUNDARY]);
    // JTS:
    // JTS:     return hasPointInCommon &&
    // JTS:         matrix[Location.INTERIOR][Location.EXTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.BOUNDARY][Location.EXTERIOR] == Dimension.FALSE;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Tests whether the argument dimensions are equal and
    // JTS:    *  this matrix matches the pattern <tt>[T*F**FFF*]</tt>.
    // JTS:    *  <p>
    // JTS:    *  <b>Note:</b> This pattern differs from the one stated in
    // JTS:    *  <i>Simple feature access - Part 1: Common architecture</i>.
    // JTS:    *  That document states the pattern as <tt>[TFFFTFFFT]</tt>.  This would
    // JTS:    *  specify that
    // JTS:    *  two identical <tt>POINT</tt>s are not equal, which is not desirable behaviour.
    // JTS:    *  The pattern used here has been corrected to compute equality in this situation.
    // JTS:    *
    // JTS:    *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
    // JTS:    *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
    // JTS:    *@return                       <code>true</code> if the two <code>Geometry</code>s
    // JTS:    *      related by this matrix are equal; the
    // JTS:    *      <code>Geometry</code>s must have the same dimension to be equal
    // JTS:    */
    // JTS:   public boolean isEquals(int dimensionOfGeometryA, int dimensionOfGeometryB) {
    // JTS:     if (dimensionOfGeometryA != dimensionOfGeometryB) {
    // JTS:       return false;
    // JTS:     }
    // JTS:     return isTrue(matrix[Location.INTERIOR][Location.INTERIOR]) &&
    // JTS:         matrix[Location.INTERIOR][Location.EXTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.BOUNDARY][Location.EXTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.EXTERIOR][Location.INTERIOR] == Dimension.FALSE &&
    // JTS:         matrix[Location.EXTERIOR][Location.BOUNDARY] == Dimension.FALSE;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Tests if this matrix matches
    // JTS:    *  <UL>
    // JTS:    *    <LI><tt>[T*T***T**]</tt> (for two points or two surfaces)
    // JTS:    *    <LI><tt>[1*T***T**]</tt> (for two curves)
    // JTS:    *  </UL>.
    // JTS:    *
    // JTS:    *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
    // JTS:    *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
    // JTS:    *@return                       <code>true</code> if the two <code>Geometry</code>s
    // JTS:    *      related by this matrix overlap. For this
    // JTS:    *      function to return <code>true</code>, the <code>Geometry</code>s must
    // JTS:    *      be two points, two curves or two surfaces.
    // JTS:    */
    // JTS:   public boolean isOverlaps(int dimensionOfGeometryA, int dimensionOfGeometryB) {
    // JTS:     if ((dimensionOfGeometryA == Dimension.P && dimensionOfGeometryB == Dimension.P) ||
    // JTS:         (dimensionOfGeometryA == Dimension.A && dimensionOfGeometryB == Dimension.A)) {
    // JTS:       return isTrue(matrix[Location.INTERIOR][Location.INTERIOR])
    // JTS:           && isTrue(matrix[Location.INTERIOR][Location.EXTERIOR])
    // JTS:           && isTrue(matrix[Location.EXTERIOR][Location.INTERIOR]);
    // JTS:     }
    // JTS:     if (dimensionOfGeometryA == Dimension.L && dimensionOfGeometryB == Dimension.L) {
    // JTS:       return matrix[Location.INTERIOR][Location.INTERIOR] == 1
    // JTS:          && isTrue(matrix[Location.INTERIOR][Location.EXTERIOR])
    // JTS:          && isTrue(matrix[Location.EXTERIOR][Location.INTERIOR]);
    // JTS:     }
    // JTS:     return false;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    * Tests whether this matrix matches the given matrix pattern.
    // JTS:    *
    // JTS:    *@param  pattern A pattern containing nine dimension symbols with which to
    // JTS:    *      compare the entries of this matrix. Possible
    // JTS:    *      symbol values are <code>{T, F, * , 0, 1, 2}</code>.
    // JTS:    *@return <code>true</code> if this matrix matches the pattern
    // JTS:    */
    // JTS:   public boolean matches(String pattern) {
    // JTS:     if (pattern.length() != 9) {
    // JTS:       throw new IllegalArgumentException("Should be length 9: " + pattern);
    // JTS:     }
    // JTS:     for (int ai = 0; ai < 3; ai++) {
    // JTS:       for (int bi = 0; bi < 3; bi++) {
    // JTS:         if (!matches(matrix[ai][bi], pattern.charAt(3 * ai +
    // JTS:             bi))) {
    // JTS:           return false;
    // JTS:         }
    // JTS:       }
    // JTS:     }
    // JTS:     return true;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Transposes this IntersectionMatrix.
    // JTS:    *
    // JTS:    *@return    this <code>IntersectionMatrix</code> as a convenience
    // JTS:    */
    // JTS:   public IntersectionMatrix transpose() {
    // JTS:     int temp = matrix[1][0];
    // JTS:     matrix[1][0] = matrix[0][1];
    // JTS:     matrix[0][1] = temp;
    // JTS:     temp = matrix[2][0];
    // JTS:     matrix[2][0] = matrix[0][2];
    // JTS:     matrix[0][2] = temp;
    // JTS:     temp = matrix[2][1];
    // JTS:     matrix[2][1] = matrix[1][2];
    // JTS:     matrix[1][2] = temp;
    // JTS:     return this;
    // JTS:   }
    // JTS:
    // JTS:   /**
    // JTS:    *  Returns a nine-character <code>String</code> representation of this <code>IntersectionMatrix</code>
    // JTS:    *  .
    // JTS:    *
    // JTS:    *@return    the nine dimension symbols of this <code>IntersectionMatrix</code>
    // JTS:    *      in row-major order.
    // JTS:    */
    // JTS:   public String toString() {
    // JTS:     StringBuilder builder = new StringBuilder("123456789");
    // JTS:     for (int ai = 0; ai < 3; ai++) {
    // JTS:       for (int bi = 0; bi < 3; bi++) {
    // JTS:         builder.setCharAt(3 * ai + bi, Dimension.toDimensionSymbol(matrix[ai][bi]));
    // JTS:       }
    // JTS:     }
    // JTS:     return builder.toString();
    // JTS:   }
    // JTS: }
    // JTS:  */
}

impl std::str::FromStr for IntersectionMatrix {
    type Err = InvalidInput;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut im = IntersectionMatrix::empty();
        im.set_at_least_from_string(str)?;
        Ok(im)
    }
}
