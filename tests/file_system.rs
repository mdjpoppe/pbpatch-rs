use std::assert_matches;

use pbpatch::{
    message_wrapper::{RepeatedMessageWrapper, SingularMessageWrapper},
    node::{Cardinality, Scalar},
    path::{PathComponent, PathRef},
    ref_cast::RefCast,
    reference::{RefMut, message::MessageRefMut},
    resolve::{ResolveError, ResolveMut, ResolveMutByRef},
};

#[derive(Debug, Eq, PartialEq)]
struct Metadata {
    /// tag: 1
    created_at: u64,

    /// tag: 2
    modified_at: Option<u64>,
}

#[derive(Debug, Eq, PartialEq)]
struct File {
    /// tag: 1
    metadata: Metadata,

    /// tag: 2
    size: u64,

    /// tag: 3
    blocks: Vec<u64>,
}

#[derive(Debug, Eq, PartialEq)]
struct Directory {
    /// tag: 1
    metadata: Metadata,

    /// tag: 2
    files: Vec<File>,
}

impl ResolveMutByRef for Metadata {
    fn resolve_mut_by_ref<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        let [head, tail @ ..] = path else {
            return Ok(RefMut::Message(MessageRefMut::Singular(
                SingularMessageWrapper::ref_cast_mut(self),
            )));
        };

        let PathComponent::Field { tag } = head else {
            return Err(ResolveError::InvalidComponentType);
        };

        match tag {
            1 => RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(&mut self.created_at)))
                .resolve_mut(tail),
            2 => RefMut::Scalar(Scalar::Uint64(Cardinality::Optional(&mut self.modified_at)))
                .resolve_mut(tail),
            _ => Err(ResolveError::UnknownField),
        }
    }
}

impl ResolveMutByRef for File {
    fn resolve_mut_by_ref<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        let [head, tail @ ..] = path else {
            return Ok(RefMut::Message(MessageRefMut::Singular(
                SingularMessageWrapper::ref_cast_mut(self),
            )));
        };

        let PathComponent::Field { tag } = head else {
            return Err(ResolveError::InvalidComponentType);
        };

        match tag {
            1 => self.metadata.resolve_mut(tail),
            2 => RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(&mut self.size)))
                .resolve_mut(tail),
            3 => RefMut::Scalar(Scalar::Uint64(Cardinality::Repeated(&mut self.blocks)))
                .resolve_mut(tail),
            _ => Err(ResolveError::UnknownField),
        }
    }
}

impl ResolveMutByRef for Directory {
    fn resolve_mut_by_ref<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
        let [head, tail @ ..] = path else {
            return Ok(RefMut::Message(MessageRefMut::Singular(
                SingularMessageWrapper::ref_cast_mut(self),
            )));
        };

        let PathComponent::Field { tag } = head else {
            return Err(ResolveError::InvalidComponentType);
        };

        match tag {
            1 => self.metadata.resolve_mut(tail),
            2 => RepeatedMessageWrapper::ref_cast_mut(&mut self.files).resolve_mut(tail),
            _ => Err(ResolveError::UnknownField),
        }
    }
}

fn dummy_directory() -> Directory {
    Directory {
        metadata: Metadata {
            created_at: 100,
            modified_at: Some(200),
        },
        files: vec![
            File {
                metadata: Metadata {
                    created_at: 110,
                    modified_at: Some(120),
                },
                size: 8192,
                blocks: vec![100, 101],
            },
            File {
                metadata: Metadata {
                    created_at: 130,
                    modified_at: None,
                },
                size: 4096,
                blocks: vec![205],
            },
            File {
                metadata: Metadata {
                    created_at: 140,
                    modified_at: Some(150),
                },
                size: 0,
                blocks: vec![],
            },
        ],
    }
}

#[test]
fn test() {
    let mut directory = dummy_directory();
    assert_matches!(
        directory.resolve_mut(&[]),
        Ok(RefMut::Message(MessageRefMut::Singular(_))),
    );
    assert_eq!(
        directory.resolve_mut(&[PathComponent::Field { tag: 1 }]), // Directory::metadata
        Ok(RefMut::Message(MessageRefMut::Singular(
            SingularMessageWrapper::ref_cast_mut(&mut Metadata {
                created_at: 100,
                modified_at: Some(200)
            })
        ))),
    );
    assert_matches!(
        directory.resolve_mut(&[
            PathComponent::Field { tag: 1 }, // Directory::metadata
            PathComponent::Field { tag: 1 }, // Metadata::created_at
        ]),
        Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(100)))),
    );
    assert_matches!(
        directory.resolve_mut(&[
            PathComponent::Field { tag: 1 }, // Directory::metadata
            PathComponent::Field { tag: 2 }, // Metadata::modified_at
        ]),
        Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Optional(Some(
            200
        ))))),
    );
    assert_matches!(
        directory.resolve_mut(&[
            PathComponent::Field { tag: 2 },      // Directory::files
            PathComponent::Repeated { index: 0 }, // Directory::files[0]
            PathComponent::Field { tag: 2 },      // File::size
        ]),
        Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(8192)))),
    );
    assert_matches!(
        directory.resolve_mut(&[
            PathComponent::Field { tag: 2 },      // Directory::files
            PathComponent::Repeated { index: 0 }, // Directory::files[0]
            PathComponent::Field { tag: 3 },      // File::blocks
        ]),
        Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Repeated(_)))),
    );
}

#[test]
fn modify() {
    let mut directory = dummy_directory();
    let path = &[
        PathComponent::Field { tag: 2 },      // Directory::files
        PathComponent::Repeated { index: 0 }, // Directory::files[0]
        PathComponent::Field { tag: 2 },      // File::size
    ];

    let Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(size)))) =
        directory.resolve_mut(path)
    else {
        panic!()
    };

    assert_eq!(*size, 8192);

    *size = 1024;

    assert_matches!(
        directory.resolve_mut(path),
        Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(1024)))),
    );
}
