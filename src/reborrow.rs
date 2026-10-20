pub trait ReborrowMut<'long> {
    type ReborrowedMut<'short>
    where
        'long: 'short,
        Self: 'short;

    fn reborrow_mut<'short>(&'short mut self) -> Self::ReborrowedMut<'short>
    where
        'long: 'short;
}

impl<'long, T: 'long + ?Sized> ReborrowMut<'long> for &'long mut T {
    type ReborrowedMut<'short>
        = &'short mut T
    where
        'long: 'short,
        Self: 'short;

    fn reborrow_mut<'short>(&'short mut self) -> Self::ReborrowedMut<'short>
    where
        'long: 'short,
    {
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
        impl<'long $($(, $generic: 'long $(+ $first_trait $(+ $other_trait)*)?)+)?> $crate::reborrow::ReborrowMut<'long> for $enum<'long $($(, $generic)+)?> {
            type ReborrowedMut<'short> = $enum<'short $($(, $generic)+)?>
            where
                'long: 'short,
                Self: 'short;

            fn reborrow_mut<'short>(&'short mut self) -> Self::ReborrowedMut<'short>
            where
                'long: 'short
            {
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
