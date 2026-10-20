//! This crate provides lightweight reflection and patching capabilities for
//! Protocol Buffers (Protobuf).
//! **Note:** this is an early-stage prototype.
//!
//! A main concept of pbpatch is a [`Node`](crate::node::Node),
//! which is a slight generalization of a *field* in Protocol Buffers.
//! In addition to fields, a `Node` can also refer to a "root" message (i.e.,
//! one that is not a field of some other message), or a single element of a
//! repeated field.
//!
//! Given a root `Node`, a [`Path`](crate::path::Path) refers to some (possibly
//! nested) `Node` within the root `Node`.
//! [`ResolveMut::resolve_mut`](crate::resolve::ResolveMut::resolve_mut)
//! can be used to resolve that path to a [`RefMut`](crate::reference::RefMut),
//! which represents a mutable reference to the designated node.
//!
//! # Example
//!
//! ```
//! # use std::assert_matches;
//! #
//! # use pbpatch::node::{Cardinality, Scalar};
//! # use pbpatch::path::{Path, PathComponent, PathRef};
//! # use pbpatch::reference::RefMut;
//! # use pbpatch::resolve::{ResolveError, ResolveMut};
//! #
//! # #[derive(Default)]
//! struct Metadata {
//!     created_at: u64,          // tag: 1
//!     modified_at: Option<u64>, // tag: 2
//! }
//!
//! # #[derive(Default)]
//! struct File {
//!     metadata: Metadata, // tag: 1
//!     size: u64,          // tag: 2
//! }
//!
//! # #[derive(Default)]
//! struct Directory {
//!     metadata: Metadata, // tag: 1
//!     files: Vec<File>,   // tag: 2
//! }
//!
//! // hidden `ResolveMut` impls
//! # impl ResolveMut for Directory {
//! #     fn resolve_mut<'a>(&'a mut self, path: PathRef<'_>) -> Result<RefMut<'a>, ResolveError> {
//! #         // hard-coded result for this example
//! #         Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(
//! #             &mut self.files[0].size
//! #         ))))
//! #     }
//! # }
//!
//! let mut directory = Directory {
//!     files: vec![
//!         File { size: 8192, ..Default::default() },
//!         File { size: 4096, ..Default::default() },
//!     ],
//!     ..Default::default()
//! };
//!
//! assert_matches!(
//!     directory.resolve_mut(&[
//!         PathComponent::Field { tag: 2 },      // Directory::files
//!         PathComponent::Repeated { index: 0 }, // Directory::files[0]
//!         PathComponent::Field { tag: 2 },      // File::size
//!     ]),
//!     Ok(RefMut::Scalar(Scalar::Uint64(Cardinality::Singular(8192)))),
//! );
//! ```

pub mod message_wrapper;
pub mod node;
pub mod path;
pub mod reborrow;
pub mod reference;
pub mod resolve;

#[doc(hidden)]
pub mod dyn_partial_eq;

mod match_variants;
mod shorten;

#[doc(hidden)]
pub use ref_cast;
