use std::{fmt::Display, ops::Deref};

use super::private::Sealed;

/// A type-level group of booleans.
pub trait Boolean: Sealed {}

/// Witnesses a type that is `true`.
pub trait Truth: Boolean {}

/// Witnesses a type that is `false`.
pub trait Falsity: Boolean {}

/// Type-level boolean corresponding to `true`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct True;

/// Type-level boolean corresponding to `false`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct False;

impl Truth for True {}
impl Falsity for False {}

impl Boolean for True {}
impl Boolean for False {}

impl Deref for True {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &true
    }
}

impl Deref for False {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &false
    }
}

impl Display for True {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "True")
    }
}

impl Display for False {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "False")
    }
}
