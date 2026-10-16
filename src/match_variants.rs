macro_rules! match_variants {
    (
        $expr:expr,
        $enum:ident::{ $($variant:ident),+ $(,)? },
        $generic_variant:ident $args:tt => $body:expr $(,)?
    ) => {
        match $expr {
            $(
                $enum::$variant $args => {
                    #[allow(non_snake_case, unused_variables)]
                    let $generic_variant = $enum::$variant;
                    $body
                }
            )+
        }
    };
}

pub(crate) use match_variants;
