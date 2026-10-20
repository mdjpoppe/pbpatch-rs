use std::{fmt::Debug, vec::Vec};

use ref_cast::RefCast;

use crate::{
    path::{PathComponent, PathRef},
    reference::{
        RefMut,
        message::{
            MessageRefMut, OptionalMessageRefMut, RepeatedMessageRefMut, SingularMessageRefMut,
        },
    },
    resolve::{ResolveError, ResolveMut},
};

#[derive(Debug, PartialEq, RefCast)]
#[repr(transparent)]
pub struct SingularMessageWrapper<M>(pub M);

impl<M: ResolveMut> ResolveMut for SingularMessageWrapper<M> {
    fn resolve_mut<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        self.0.resolve_mut(path)
    }
}

impl<M: 'static + ResolveMut + Debug + PartialEq> SingularMessageRefMut
    for SingularMessageWrapper<M>
{
}

#[derive(Debug, PartialEq, RefCast)]
#[repr(transparent)]
pub struct OptionalMessageWrapper<M>(pub Option<M>);

impl<M: 'static + ResolveMut + Debug + PartialEq> ResolveMut for OptionalMessageWrapper<M> {
    fn resolve_mut<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        if path.is_empty() {
            return Ok(RefMut::Message(MessageRefMut::Optional(self)));
        }

        match &mut self.0 {
            Some(message) => message.resolve_mut(path),
            None => Err(ResolveError::UnsetMessage),
        }
    }
}

impl<M: 'static + ResolveMut + Debug + PartialEq> OptionalMessageRefMut
    for OptionalMessageWrapper<M>
{
}

#[derive(Debug, PartialEq, RefCast)]
#[repr(transparent)]
pub struct RepeatedMessageWrapper<M>(pub Vec<M>);

impl<M: 'static + ResolveMut + Debug + PartialEq> ResolveMut for RepeatedMessageWrapper<M> {
    fn resolve_mut<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        let [head, tail @ ..] = path else {
            return Ok(RefMut::Message(MessageRefMut::Repeated(
                RepeatedMessageWrapper::ref_cast_mut(&mut self.0),
            )));
        };

        match head {
            PathComponent::Repeated { index } => {
                let index = usize::try_from(*index).map_err(|_| ResolveError::IndexOutOfBounds)?;
                let element = self
                    .0
                    .get_mut(index)
                    .ok_or(ResolveError::IndexOutOfBounds)?;
                ResolveMut::resolve_mut(element, tail)
            }
            _ => Err(ResolveError::InvalidComponentType),
        }
    }
}

impl<M: 'static + ResolveMut + Debug + PartialEq> RepeatedMessageRefMut
    for RepeatedMessageWrapper<M>
{
}
