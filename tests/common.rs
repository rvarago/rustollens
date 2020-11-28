use rustollens::boolean::{Boolean, Falsity, Truth};

pub fn deduce_boolean<B: Boolean>() {}

pub fn deduce_truth<T: Truth>() {
    deduce_boolean::<T>()
}

pub fn deduce_falsity<F: Falsity>() {
    deduce_boolean::<F>()
}
