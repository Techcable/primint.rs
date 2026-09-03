use primint::{NonMax, UnsignedPrimInt};

#[test]
fn basic_roundtrip() {
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
