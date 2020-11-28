mod common;

use common::deduce_boolean;
use rustollens::boolean::{False, True};

#[test]
fn type_level_booleans_can_be_grouped() {
    deduce_boolean::<True>();
    deduce_boolean::<False>();
}

#[test]
fn type_level_booleans_are_convertible_to_runtime_booleans() {
    assert!(*True);
    assert!(!*False);
}

#[test]
fn type_level_booleans_can_be_shown() {
    assert_eq!(True.to_string(), "True");
    assert_eq!(False.to_string(), "False");
}
