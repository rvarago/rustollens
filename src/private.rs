use super::boolean::{False, True};

pub trait Sealed {}

impl Sealed for True {}

impl Sealed for False {}
