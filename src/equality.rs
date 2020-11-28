use super::{
    boolean::{False, True},
    private::Sealed,
};

/// Type-level Equality.
pub trait Equality<L>: Sealed {
    /// Whether `Self` equals to `L`.
    type Output;
}

/// Syntactic sugar for Equality.
pub type Eql<L, R> = <L as Equality<R>>::Output;

impl Equality<False> for False {
    type Output = True;
}

impl Equality<True> for False {
    type Output = False;
}

impl Equality<False> for True {
    type Output = False;
}

impl Equality<True> for True {
    type Output = True;
}
