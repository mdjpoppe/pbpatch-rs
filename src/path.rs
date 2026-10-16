/// Designates a [`Node`](crate::node::Node) in a (possibly nested) root
/// `Node`.
pub type Path = Vec<PathComponent>;

/// Reference to a [`Path`].
pub type PathRef<'a> = &'a [PathComponent];

/// Single component along a [`Path`].
#[derive(Clone, Debug, PartialEq)]
pub enum PathComponent {
    Field { tag: u32 },
    Repeated { index: u32 },
}
