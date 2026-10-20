use crate::{
    match_variants::match_variants,
    node::Node,
    path::{PathComponent, PathRef},
    reborrow::ReborrowMut,
    reference::{
        RefMut,
        message::MessageRefMut,
        scalar::{GenericScalarRefMut, ScalarRefMut},
    },
    shorten::Shorten,
};

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ResolveError {
    /// A path component was expected, but the supplied type is invalid.
    ///
    /// Example: [`PathComponent::Repeated`] was provided where
    /// [`PathComponent::Field`] was expected.
    InvalidComponentType,

    /// A path component was supplied, but none was expected.
    ///
    /// Example: a path component following a singular scalar field.
    UnexpectedComponent,

    /// Path component refers to an unknown field.
    UnknownField,

    /// Path component contains an out-of-bounds index for a repeated node.
    IndexOutOfBounds,

    /// An intermediate path component (i.e., any but the last) refers to a
    /// message field that is not set.
    UnsetMessage,

    /// Path component refers to a node or uses a feature which is not supported
    /// (yet).
    Unsupported,
}

pub trait ResolveMut {
    fn resolve_mut<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError>;
}

pub trait ResolveMutOwned {
    fn resolve_mut_owned<'a>(self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError>
    where
        Self: 'a;
}

impl ResolveMut for RefMut<'_> {
    fn resolve_mut<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        self.reborrow_mut().resolve_mut_owned(path)
    }
}

impl<'s> ResolveMutOwned for RefMut<'s> {
    fn resolve_mut_owned<'a>(self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError>
    where
        's: 'a,
    {
        match self.shorten() {
            Node::Scalar(scalar) => scalar.resolve_mut_owned(path),
            Node::Message(message) => message.resolve_mut_owned(path),
        }
    }
}

impl<'s> ResolveMutOwned for ScalarRefMut<'s> {
    fn resolve_mut_owned<'a>(self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError>
    where
        's: 'a,
    {
        match_variants!(
            self.shorten(),
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
            },
            Var(scalar) => resolve_generic_scalar_ref_mut(scalar, path).map(Var)
        )
        .map(Node::Scalar)
    }
}

fn resolve_generic_scalar_ref_mut<'a, T: 'static>(
    scalar: GenericScalarRefMut<'a, T>,
    path: PathRef<'_>,
) -> Result<GenericScalarRefMut<'a, T>, ResolveError> {
    let [head, tail @ ..] = path else {
        return Ok(scalar);
    };

    match scalar {
        GenericScalarRefMut::Singular(_) | GenericScalarRefMut::Optional(_) => {
            Err(ResolveError::UnexpectedComponent)
        }
        GenericScalarRefMut::Repeated(repeated) => {
            let PathComponent::Repeated { index } = head else {
                return Err(ResolveError::InvalidComponentType);
            };
            let index = usize::try_from(*index).map_err(|_| ResolveError::IndexOutOfBounds)?;
            let element = repeated
                .get_mut(index)
                .ok_or(ResolveError::IndexOutOfBounds)?;
            resolve_generic_scalar_ref_mut(GenericScalarRefMut::Singular(element), tail)
        }
    }
}

impl<'s> ResolveMutOwned for MessageRefMut<'s> {
    fn resolve_mut_owned<'a>(self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError>
    where
        's: 'a,
    {
        match self {
            MessageRefMut::Singular(singular) => singular.resolve_mut(path),
            MessageRefMut::Optional(optional) => optional.resolve_mut(path),
            MessageRefMut::Repeated(repeated) => repeated.resolve_mut(path),
        }
    }
}
