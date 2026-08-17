//! Trait bounds used for [`crate::PrimitiveInt`].

macro_rules! convert_prim_int {
    ($($target:ident),+ $(,)?) => {
        #[doc(hidden)]
        pub trait ConvertPrimInts:
            Sized
            $(+ TryFrom<$target>)*
            $(+ TryInto<$target>)*
            $(+ crate::private::WrappingCastFrom<$target>)*
        {
        }
    };
}
convert_prim_int! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

macro_rules! int_ops {
    (@regular [$($regular:ident),+ $(,)?] @assign [$($assign:ident),+ $(,)?]) => {
        #[doc(hidden)]
        pub trait IntOps:
            Sized
            $(+ core::ops::$regular::<Output=Self>)*
            $(+ core::ops::$assign)*
        {}
    };
}
int_ops!(
    @regular [
        Add,
        BitAnd,
        BitOr,
        BitXor,
        Div,
        Mul,
        Not,
        Rem,
        Shl,
        Shr,
        Sub,
    ]
    @assign [
        AddAssign,
        BitAndAssign,
        BitOrAssign,
        BitXorAssign,
        DivAssign,
        MulAssign,
        RemAssign,
        ShlAssign,
        ShrAssign,
        SubAssign,
    ]
);

macro_rules! maybe_trait_bound {
    ($name:ident, cfg($flag:meta), $bound:path $(,)?) => {
        #[cfg($flag)]
        #[doc(hidden)]
        pub trait $name: $bound {}
        #[cfg(not($flag))]
        #[doc(hidden)]
        pub trait $name {}
        #[cfg($flag)]
        impl<T: $bound> $name for T {}
        #[cfg(not($flag))]
        impl<T> $name for T {}
    };
}

pub mod num_trait_02 {
    maybe_trait_bound!(PrimInt, cfg(feature = "num-traits-02"), num_traits_02::PrimInt);
    maybe_trait_bound!(Unsigned, cfg(feature = "num-traits-02"), num_traits_02::Unsigned);
    maybe_trait_bound!(Signed, cfg(feature = "num-traits-02"), num_traits_02::Signed);
}

pub mod bytemuck {
    maybe_trait_bound!(Pod, cfg(feature = "bytemuck"), bytemuck::Pod);
    maybe_trait_bound!(Contiguous, cfg(feature = "bytemuck"), bytemuck::Contiguous);
}

pub mod serde {
    maybe_trait_bound!(Serialize, cfg(feature = "serde"), serde::Serialize);
    maybe_trait_bound!(DeserializeOwned, cfg(feature = "serde"), serde::de::DeserializeOwned);
}
