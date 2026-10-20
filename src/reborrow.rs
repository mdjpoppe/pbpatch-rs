/// Helper trait for converting `&'short mut R<'long>` to `R<'short>` for
/// mutable reference-like types `R`.
///
/// The `_OutlivesShort = &'short Self` parameter is used to imply
/// `Self: 'short`.
/// Thus, `T: ReborrowMut<'short>` automatically implies `T: 'short`.
/// This is helpful in Higher-Rank Trait Bounds (HRTBs) such as
/// `for<'short> T: ReborrowMut<'short>`, where it is not possible to express
/// `T: 'short` directly.
pub trait ReborrowMut<'short, _OutlivesShort = &'short Self> {
    type ReborrowedMut;

    fn reborrow_mut(&'short mut self) -> Self::ReborrowedMut;
}

// It is tempting to add `'long: 'short` here, but then
// `for<'short> T: ReborrowMut<'short>` would require `'long = 'static`.
// In any case, the signature of `reborrow_mut` implies that `'long: 'short`.
impl<'short, 'long, T: 'long + ?Sized> ReborrowMut<'short> for &'long mut T {
    type ReborrowedMut = &'short mut T;

    fn reborrow_mut(&'short mut self) -> Self::ReborrowedMut {
        self
    }
}

macro_rules! generate_reborrow_mut {
    (
        $enum:ident
        $(< $(
            $generic:ident $(: $first_trait:ident $(+ $other_trait:ident)*)?
        ),+ >)?
        ::{ $($variant:ident),* $(,)? }
    ) => {
        impl<'short, 'long $($(, $generic: 'long $(+ $first_trait $(+ $other_trait)*)?)+)?> $crate::reborrow::ReborrowMut<'short> for $enum<'long $($(, $generic)+)?> {
            type ReborrowedMut = $enum<'short $($(, $generic)+)?>;

            fn reborrow_mut(&'short mut self) -> Self::ReborrowedMut {
                match self {
                    $(
                        $enum::$variant(v) => $enum::$variant($crate::reborrow::ReborrowMut::reborrow_mut(v)),
                    )*
                }
            }
        }
    };
}

pub(crate) use generate_reborrow_mut;
