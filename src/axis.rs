/// The 2D coordinate axes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Axis2 {
    /// The x-axis.
    X,
    /// The y-axis.
    Y,
}

impl Axis2 {
    /// The coordinate axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Returns the next axis: `X -> Y`.
    #[inline]
    #[must_use]
    pub const fn next(self) -> Self {
        use Axis2::*;
        match self {
            X => Y,
            Y => X,
        }
    }
}

/// The 3D coordinate axes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Axis3 {
    /// The x-axis.
    X,
    /// The y-axis.
    Y,
    /// The z-axis.
    Z,
}

impl Axis3 {
    /// The coordinate axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Returns the next axis: `X -> Y -> Z`.
    #[inline]
    #[must_use]
    pub const fn next(self) -> Self {
        use Axis3::*;
        match self {
            X => Y,
            Y => Z,
            Z => X,
        }
    }

    /// Returns the previous axis: `Z -> Y -> X`.
    #[inline]
    #[must_use]
    pub const fn prev(self) -> Self {
        use Axis3::*;
        match self {
            X => Z,
            Y => X,
            Z => Y,
        }
    }
}

/// The 4D coordinate axes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Axis4 {
    /// The x-axis.
    X,
    /// The y-axis.
    Y,
    /// The z-axis.
    Z,
    /// The w-axis.
    W,
}

impl Axis4 {
    /// The coordinate axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

    /// Returns the next axis: `X -> Y -> Z -> W`.
    #[inline]
    #[must_use]
    pub const fn next(self) -> Self {
        use Axis4::*;
        match self {
            X => Y,
            Y => Z,
            Z => W,
            W => X,
        }
    }

    /// Returns the previous axis: `W -> Z -> Y -> X`.
    #[inline]
    #[must_use]
    pub const fn prev(self) -> Self {
        use Axis4::*;
        match self {
            X => W,
            Y => X,
            Z => Y,
            W => Z,
        }
    }
}

crate::derive_index_traits!(@bool: glam::BVec2, Axis2 { X => x, Y => y });
crate::derive_index_traits!(@bool: glam::BVec3, Axis3 { X => x, Y => y, Z => z });
crate::derive_index_traits!(@bool: glam::BVec4, Axis4 { X => x, Y => y, Z => z, W => w });
