use core::any::Any;

pub trait DynPartialEq: Any {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + Any> DynPartialEq for T {
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>().is_some_and(|other| self == other)
    }
}
