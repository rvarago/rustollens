mod common;

use common::{deduce_falsity, deduce_truth};
use rustollens::boolean::{False, True};
use rustollens::connective::{And, Bimp, Imp, Not, Or, Xor};

#[test]
fn negation_holds_for_all_permutations_of_booleans() {
    deduce_truth::<Not<False>>();
    deduce_falsity::<Not<True>>();
}

#[test]
fn double_negation_reduces_to_the_initial_type() {
    deduce_falsity::<Not<Not<False>>>();
    deduce_truth::<Not<Not<True>>>();
}

#[test]
fn conjunction_holds_for_all_permutations_of_booleans() {
    deduce_falsity::<And<False, False>>();
    deduce_falsity::<And<False, True>>();
    deduce_falsity::<And<True, False>>();
    deduce_truth::<And<True, True>>();
}

#[test]
fn disjunction_holds_for_all_permutations_of_booleans() {
    deduce_falsity::<Or<False, False>>();
    deduce_truth::<Or<False, True>>();
    deduce_truth::<Or<True, False>>();
    deduce_truth::<Or<True, True>>();
}

#[test]
fn exclusive_or_holds_for_all_permutations_of_booleans() {
    deduce_falsity::<Xor<False, False>>();
    deduce_truth::<Xor<False, True>>();
    deduce_truth::<Xor<True, False>>();
    deduce_falsity::<Xor<True, True>>();
}

#[test]
fn implication_holds_for_all_permutations_of_booleans() {
    deduce_truth::<Imp<False, False>>();
    deduce_truth::<Imp<False, True>>();
    deduce_falsity::<Imp<True, False>>();
    deduce_truth::<Imp<True, True>>();
}

#[test]
fn bi_implication_holds_for_all_permutations_of_booleans() {
    deduce_truth::<Bimp<False, False>>();
    deduce_falsity::<Bimp<False, True>>();
    deduce_falsity::<Bimp<True, False>>();
    deduce_truth::<Bimp<True, True>>();
}
