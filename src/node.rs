pub trait NodeFamily {
    type ScalarFamily: ScalarFamily;

    type MessageType;
}

#[derive(derive_more::Debug, derive_more::Eq, derive_more::PartialEq)]
pub enum Node<F: NodeFamily> {
    Scalar(Scalar<F::ScalarFamily>),
    Message(F::MessageType),
}

pub trait ScalarFamily {
    type Type<T: 'static>;
}

#[derive(derive_more::Debug, derive_more::Eq, derive_more::PartialEq)]
pub enum Scalar<F: ScalarFamily> {
    Double(F::Type<f64>),
    Float(F::Type<f32>),
    Int32(F::Type<i32>),
    Int64(F::Type<i64>),
    Uint32(F::Type<u32>),
    Uint64(F::Type<u64>),
    Sint32(F::Type<i32>),
    Sint64(F::Type<i64>),
    Fixed32(F::Type<u32>),
    Fixed64(F::Type<u64>),
    Sfixed32(F::Type<i32>),
    Sfixed64(F::Type<i64>),
    Bool(F::Type<bool>),
    String(F::Type<String>),
    Bytes(F::Type<Vec<u8>>),
    Enumeration(F::Type<i32>),
}

pub trait CardinalityFamily {
    type SingularType;
    type OptionalType;
    type RepeatedType;
}

#[derive(derive_more::Debug, derive_more::Eq, derive_more::PartialEq)]
pub enum Cardinality<F: CardinalityFamily> {
    Singular(F::SingularType),
    Optional(F::OptionalType),
    Repeated(F::RepeatedType),
}
