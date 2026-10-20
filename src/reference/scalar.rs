use std::marker::PhantomData;

use crate::{
    node::{Cardinality, CardinalityFamily, Scalar, ScalarFamily},
    reborrow::generate_reborrow_mut,
    shorten::generate_shorten,
};

pub type ScalarRefMut<'a> = Scalar<ScalarRefMutFamily<'a>>;

generate_shorten!(
    ScalarRefMut::{
        Double,
        Float,
        Int32,
        Int64,
        Uint32,
        Uint64,
        Sint32,
        Sint64,
        Fixed32,
        Fixed64,
        Sfixed32,
        Sfixed64,
        Bool,
        String,
        Bytes,
        Enumeration,
    }
);
generate_reborrow_mut!(
    ScalarRefMut::{
        Double,
        Float,
        Int32,
        Int64,
        Uint32,
        Uint64,
        Sint32,
        Sint64,
        Fixed32,
        Fixed64,
        Sfixed32,
        Sfixed64,
        Bool,
        String,
        Bytes,
        Enumeration,
    }
);

pub struct ScalarRefMutFamily<'a>(PhantomData<&'a ()>);

impl<'a> ScalarFamily for ScalarRefMutFamily<'a> {
    type Type<T: 'static> = GenericScalarRefMut<'a, T>;
}

pub type GenericScalarRefMut<'a, T> = Cardinality<ScalarRefMutCardinalityFamily<'a, T>>;

generate_shorten!(GenericScalarRefMut<T>::{Singular, Optional, Repeated});
generate_reborrow_mut!(GenericScalarRefMut<T>::{Singular, Optional, Repeated});

pub struct ScalarRefMutCardinalityFamily<'a, T>(PhantomData<(&'a (), T)>);

impl<'a, T: 'a> CardinalityFamily for ScalarRefMutCardinalityFamily<'a, T> {
    type SingularType = &'a mut T;
    type OptionalType = &'a mut Option<T>;
    type RepeatedType = &'a mut Vec<T>;
}
