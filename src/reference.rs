pub mod message;
pub mod scalar;

use core::marker::PhantomData;

use crate::{
    node::{Node, NodeFamily},
    reference::{message::MessageRefMut, scalar::ScalarRefMutFamily},
    shorten::generate_shorten,
};

pub type RefMut<'a> = Node<RefMutNodeFamily<'a>>;

generate_shorten!(RefMut::{Scalar, Message});

pub struct RefMutNodeFamily<'a>(PhantomData<&'a ()>);

impl<'a> NodeFamily for RefMutNodeFamily<'a> {
    type ScalarFamily = ScalarRefMutFamily<'a>;

    type MessageType = MessageRefMut<'a>;
}
