use std::{fmt::Debug, marker::PhantomData};

use crate::{
    dyn_partial_eq::DynPartialEq,
    node::{Cardinality, CardinalityFamily},
    resolve::ResolveMut,
    shorten::generate_shorten,
};

pub type MessageRefMut<'a> = Cardinality<MessageRefMutCardinalityFamily<'a>>;

generate_shorten!(MessageRefMut::{Singular, Optional, Repeated});

impl<'a> PartialEq for MessageRefMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        use Cardinality::{Optional, Repeated, Singular};
        match (self, other) {
            (Singular(lhs), Singular(rhs)) => lhs.dyn_eq(*rhs),
            (Optional(lhs), Optional(rhs)) => lhs.dyn_eq(*rhs),
            (Repeated(lhs), Repeated(rhs)) => lhs.dyn_eq(*rhs),
            _ => false,
        }
    }
}

pub struct MessageRefMutCardinalityFamily<'a>(PhantomData<&'a ()>);

impl<'a> CardinalityFamily for MessageRefMutCardinalityFamily<'a> {
    type SingularType = &'a mut (dyn SingularMessageRefMut + 'static);
    type OptionalType = &'a mut (dyn OptionalMessageRefMut + 'static);
    type RepeatedType = &'a mut (dyn RepeatedMessageRefMut + 'static);
}

pub trait SingularMessageRefMut: ResolveMut + DynPartialEq + Debug {}

pub trait OptionalMessageRefMut: ResolveMut + DynPartialEq + Debug {}

pub trait RepeatedMessageRefMut: ResolveMut + DynPartialEq + Debug {}
