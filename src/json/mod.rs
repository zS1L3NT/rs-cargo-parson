use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

pub mod array;
pub mod boolean;
pub mod null;
pub mod number;
pub mod object;
pub mod string;

pub trait JSONValue: Downcast + Debug {
    fn to_string(&self) -> String;
}
impl_downcast!(JSONValue);
