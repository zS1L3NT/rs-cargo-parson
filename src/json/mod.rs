use std::{any::Any, fmt::Debug};

use downcast_rs::{Downcast, impl_downcast};

pub mod array;
pub mod boolean;
pub mod null;
pub mod number;
pub mod object;
pub mod string;

pub trait JSONValue: Downcast + Debug {
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
impl_downcast!(JSONValue);