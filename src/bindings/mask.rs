use core::ops;
use crate::raw::FromRaw;

/// Interface for `glam` bitmask types.
pub trait Mask: ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::Not
    + FromRaw<Raw = Self>
{
    /// Returns true if all elements are true, false otherwise.
    fn all(self) -> bool;

    /// Returns true if any of the elements are true, false otherwise.
    fn any(self) -> bool;
}

macro_rules! impl_mask {
    ($ty:ty) => {
        impl Mask for $ty {
            fn all(self) -> bool {
                <$ty>::all(self)
            }

            fn any(self) -> bool {
                <$ty>::any(self)
            }
        }
    };
}

impl_mask!(glam::BVec2);

impl_mask!(glam::BVec3);

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math")),
))]
impl_mask!(glam::BVec3A);

impl_mask!(glam::BVec4);

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math")),
))]
impl_mask!(glam::BVec4A);