use std::cmp::Ordering;

use primint::{NonMax, UnsignedPrimInt};
use proptest::proptest;
use proptest::strategy::Strategy;

#[macro_use]
mod utils;

fn nonmax() -> impl Strategy<Value = NonMax<u32>> {
    (NonMax::<u32>::MIN.get()..=NonMax::MAX.get()).prop_map(|x| NonMax::<u32>::new(x).unwrap())
}

#[test]
fn roundtrip_basic() {
    fn roundtrip<T: UnsignedPrimInt>(val: T) {
        let item = NonMax::new(val).unwrap_or_else(|| core::panic!("Maximum value {val}"));
        assert_eq!(item.get(), val);
    }
    macro_rules! basic_roundtrip {
            ($($t:ident),+) => {
                $(roundtrip::<$t>(primint::zero());
                roundtrip::<$t>(primint::one());
                roundtrip::<$t>(2u8.into());
                roundtrip::<$t>(primint::max_value::<$t>() - 2);
                roundtrip::<$t>(primint::max_value::<$t>() - 1);)*
            };
        }
    basic_roundtrip!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn ordered_basic() {
    fn ord(x: u32, y: u32) -> Ordering {
        NonMax::new(x).unwrap().cmp(&NonMax::new(y).unwrap())
    }
    assert_eq!(ord(0, 1), Ordering::Less);
    assert_eq!(ord(1, 0), Ordering::Greater);
    assert_eq!(ord(u32::MAX - 1, 1), Ordering::Greater);
    assert_eq!(ord(u32::MAX - 1, 0), Ordering::Greater);
    assert_eq!(ord(1, 1), Ordering::Equal);
}

#[test]
fn hash_same_basic() {
    use crate::utils::hash::hashed_bytes;
    #[track_caller]
    fn check<T: UnsignedPrimInt>(value: T) {
        assert_eq!(hashed_bytes(NonMax::new(value).unwrap()), hashed_bytes(value));
    }
    macro_rules! basics {
        ($($target:ident),+) => {
            $(for x in 0u8..=3u8 {
                check::<$target>($target::MAX - 1 - $target::from(x));
                check::<$target>($target::from(x));
            })*
        };
    }
    basics!(u8, u16, u32, u64, u128, usize);
}

noncommon_proptests!(NonMax<u32> with nonmax());
