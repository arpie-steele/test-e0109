//! Minimal test.
//!
//! This project was created with `cargo +1.77 new --edition 2018 test-e0109`

/// Unobjectional trait?
pub trait UnsignedBits<U, const N: usize> {
    /// Class method to create a mask of type U based on trait parameter N.
    fn mask() -> U;
}

/// Example macro where use of $n is flagged
macro_rules! ub_prim_impl {
    ($ub:ident; $prim_type:ty, $n:literal; $max_n:literal) => {
        impl $ub<$prim_type, $n> for $prim_type {
            fn mask() -> $prim_type {
                assert!($n <= $max_n);
                if $n == $max_n {
                    return <$prim_type>::MAX;
                }
                (1 as $prim_type << (1 << $n)) - 1
            }
        }
    };
}
// The following seems to have a E0109 error at the third (varying) argument
// from rust-analyzer 0.3.2660-standalone (7c810e9994 2025-10-27) under
// VS Code,  but the code compiles with +1.77 toolset and edition 2018.
ub_prim_impl!(UnsignedBits; u8, 0; 3);
ub_prim_impl!(UnsignedBits; u8, 1; 3);
ub_prim_impl!(UnsignedBits; u8, 2; 3);
ub_prim_impl!(UnsignedBits; u8, 3; 3);

/// Demonstrate the code compiles, runs, and behaves as expected.
fn main() {
    assert_eq!(<u8 as UnsignedBits<u8, 0>>::mask(), 0x01_u8);
    assert_eq!(<u8 as UnsignedBits<u8, 1>>::mask(), 0x03_u8);
    assert_eq!(<u8 as UnsignedBits<u8, 2>>::mask(), 0x0f_u8);
    assert_eq!(<u8 as UnsignedBits<u8, 3>>::mask(), 0xff_u8);
    println!("Hello, world!");
}
