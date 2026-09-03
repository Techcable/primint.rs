#[macro_export]
macro_rules! noncommon_proptests {
    (@check_binops $a:ident, $b:ident, $first:ty, $second:ty, {$($trait_name:ident => $func:ident),+ $(,)?}) => {
        $(assert_eq!(
            <$first as $trait_name>::$func(&$a, &$b),
            <$second as $trait_name>::$func(&$a.get(), &$b.get()),
        );)*
    };
    ($target:ident<$prim:ident> with $gen:ident()) => {
        proptest! {
            #[test]
            fn ordered_same(a in $gen(), b in $gen()) {
                noncommon_proptests!(@check_binops a, b, $target<$prim>, $prim, {
                    Ord => cmp,
                    PartialOrd => partial_cmp,
                    PartialOrd => lt,
                    PartialOrd => gt,
                    PartialOrd => le,
                    PartialOrd => ge,
                    PartialEq => eq,
                    PartialEq => ne,
                });
            }
        }

        proptest! {
            #[test]
            fn hash_same(a in $gen()) {
                use $crate::utils::hash::hashed_bytes;
                assert_eq!(
                    hashed_bytes::<$target<$prim>>(a),
                    hashed_bytes::<$prim>(a.get()),
                );
            }
        }

        proptest! {
            #[test]
            fn debug_same(a in $gen()) {
                assert_eq!(
                    format!("{a:?}"),
                    format!("{:?}", a.get())
                );
            }
        }

        proptest! {
            #[test]
            fn minmax_same(a in $gen(), b in $gen()) {
                assert_eq!(
                    <$target<$prim> as Ord>::min(a, b).get(),
                    <$prim as Ord>::min(a.get(), b.get())
                );
                assert_eq!(
                    <$target<$prim> as Ord>::max(a, b).get(),
                    <$prim as Ord>::max(a.get(), b.get())
                );
            }
        }

        proptest! {
            #[test]
            fn clamp_same(x in $gen(), a in $gen(), b in $gen()) {
                // Assumes that minimax works
                let min = a.min(b);
                let max = a.max(b);
                assert_eq!(
                    <$target<$prim> as Ord>::clamp(x, min, max).get(),
                    <$prim as Ord>::clamp(x.get(), min.get(), max.get())
                );
            }
        }
    };
}
