use super::{
    boolean::{False, True},
    private::Sealed,
};

/// Type-level negation (NOT).
pub trait Negation: Sealed {
    /// Logical conclusion of the negation of `Self`.
    type Output;
}

/// Type-level conjunction (AND).
pub trait Conjunction<L>: Sealed {
    /// Logical conclusion of the conjunction of `Self` and `L`.
    type Output;
}

/// Type-level disjunction (OR).
pub trait Disjunction<L>: Sealed {
    /// Logical conclusion of the disjunction of `Self` and `L`.
    type Output;
}

/// Syntactic sugar for Negation.
pub type Not<A> = <A as Negation>::Output;

/// Syntactic sugar for Conjunction.
pub type And<L, R> = <L as Conjunction<R>>::Output;

/// Syntactic sugar for Disjunction.
pub type Or<L, R> = <L as Disjunction<R>>::Output;

/// Exclusive-OR of `L` and `R`.
pub type Xor<L, R> = Or<And<L, Not<R>>, And<Not<L>, R>>;

/// Logical implication.
pub type Imp<L, R> = Or<Not<L>, R>;

/// Logical bi-implication.
pub type Bimp<L, R> = And<Imp<L, R>, Imp<R, L>>;

impl Negation for True {
    type Output = False;
}

impl Negation for False {
    type Output = True;
}

impl Conjunction<False> for False {
    type Output = False;
}

impl Conjunction<True> for False {
    type Output = False;
}

impl Conjunction<False> for True {
    type Output = False;
}

impl Conjunction<True> for True {
    type Output = True;
}

impl Disjunction<False> for False {
    type Output = False;
}

impl Disjunction<True> for False {
    type Output = True;
}

impl Disjunction<False> for True {
    type Output = True;
}

impl Disjunction<True> for True {
    type Output = True;
}
