pub trait Shorten<'long> {
    type Shortened<'short>
    where
        'long: 'short;

    fn shorten<'short>(self) -> Self::Shortened<'short>
    where
        'long: 'short;
}

impl<'long, T: ?Sized> Shorten<'long> for &'long T {
    type Shortened<'short>
        = &'short T
    where
        'long: 'short;

    fn shorten<'short>(self) -> Self::Shortened<'short>
    where
        'long: 'short,
    {
        self
    }
}

impl<'long, T: ?Sized> Shorten<'long> for &'long mut T {
    type Shortened<'short>
        = &'short mut T
    where
        'long: 'short;

    fn shorten<'short>(self) -> Self::Shortened<'short>
    where
        'long: 'short,
    {
        self
    }
}

macro_rules! generate_shorten {
    (
        $enum:ident
        $(< $(
            $generic:ident $(: $first_trait:ident $(+ $other_trait:ident)*)?
        ),+ >)?
        ::{ $($variant:ident),* $(,)? }
    ) => {
        impl<'long $($(, $generic: 'long $(+ $first_trait $(+ $other_trait)*)?)+)?> $crate::shorten::Shorten<'long> for $enum<'long $($(, $generic)+)?> {
            type Shortened<'short> = $enum<'short $($(, $generic)+)?>
            where
                'long: 'short;

            fn shorten<'short>(self) -> Self::Shortened<'short>
            where
                'long: 'short
            {
                match self {
                    $(
                        $enum::$variant(v) => $enum::$variant($crate::shorten::Shorten::shorten(v)),
                    )*
                }
            }
        }
    };
}

pub(crate) use generate_shorten;
