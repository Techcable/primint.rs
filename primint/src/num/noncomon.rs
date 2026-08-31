//! Code common to [`crate::num::NonZero`] and [`crate::num::NonMax`].
macro_rules! fmt_delegate_traits {
    ($target:ident<$bound:ident>) => {
        fmt_delegate_traits!(
            $target<$bound> <=
            core::fmt::Display,
            // all the other numeric format traits:
            core::fmt::Binary,
            core::fmt::LowerExp,
            core::fmt::LowerHex,
            core::fmt::Octal,
            core::fmt::UpperExp,
            core::fmt::UpperHex,
        );
    };
    ($target:ident<$bound:ident> <= $($tname:path),+ $(,)?) => {
        $(
            #[doc = concat!("Unconditionally delegates to [`", stringify!($tname), "`] on [`Self::get`].")]
            impl<T: $bound> $tname for $target<T> {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    <T as $tname>::fmt(&self.get(), f)
                }
            }
        )*
    }
}

#[cfg(feature = "nonmax")]
mod nonmax {
    use crate::UnsignedPrimInt;
    use crate::num::NonMax;

    fmt_delegate_traits!(NonMax<UnsignedPrimInt>);
}

#[cfg(feature = "nonzero")]
mod nonzero {
    use crate::PrimitiveInt;
    use crate::num::NonZero;

    fmt_delegate_traits!(NonZero<PrimitiveInt>);
}
