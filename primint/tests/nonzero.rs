use primint::NonZero;
use proptest::arbitrary::Arbitrary;
use proptest::prelude::Strategy;
use proptest::proptest;

#[macro_use]
mod utils;

fn nonzero() -> impl Strategy<Value = NonZero<i32>> {
    core::num::NonZeroI32::arbitrary().prop_map(NonZero::from)
}

noncommon_proptests!(NonZero<i32> with nonzero());
