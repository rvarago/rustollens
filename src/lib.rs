#![deny(missing_docs)]

//! Type-level boolean with companion logical connectives.

/// Type-level boolean type definition,
pub mod boolean;

/// Logical connectives.
pub mod connective;

/// Propositional equality of booleans.
pub mod equality;

mod private;
