mod common;

use common::{deduce_falsity, deduce_truth};
use rustollens::boolean::{False, True};
use rustollens::equality::Eql;

#[test]
fn equality_holds_for_all_permutations_of_booleans() {
    deduce_truth::<Eql<False, False>>();
    deduce_falsity::<Eql<False, True>>();
    deduce_falsity::<Eql<True, False>>();
    deduce_truth::<Eql<True, True>>();
}
