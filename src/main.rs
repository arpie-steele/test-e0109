//! Minimal test.
//!
//! This project was created with `cargo +1.77 new --edition 2018 test-e0109`

/// Unobjectional trait.
pub trait UnsignedBits<U, const N: usize> {
    /// Class method to create a mask of type U based on trait parameter N.
    fn mask() -> U;
}

// Unobjectional implentation.
impl UnsignedBits<bool, 0> for bool {
    fn mask() -> bool {
        true
    }
}

/// Example macro where use of $n is flagged in usage.
macro_rules! ub_prim_impl {
    ($ub:ident; $prim_type:ty, $n:literal; $max_n:literal) => {
        impl $ub<$prim_type, $n> for $prim_type {
            fn mask() -> $prim_type {
                assert!($n <= $max_n);
                if $n == $max_n {
                    return <$prim_type>::MAX;
                }
                // The problem goes away if the expression `1 as $prim_type`
                // is parenthesized.... but it compiles as is, possibly due
                // to pre-parsing of the macro by the compiler.
                //
                // But rust-analyzer evaluates this as if it was just
                // string-substituted and sees `"u8 <<"` and assumes we are
                // trying treat `u8` as a generic type.
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
ub_prim_impl!(UnsignedBits; u8, 3; 3);

// Expand ub_prim_impl!(UnsignedBits; u8, 2; 3);
#[allow(clippy::assertions_on_constants, clippy::unnecessary_cast)]
impl UnsignedBits<u8, 2> for u8 {
    fn mask() -> u8 {
        assert!(2 <= 3);
        if 2 == 3 {
            return <u8>::MAX;
        }
        // The compiler requires parentheses about `1 as u8` outside of the macro.
        // And this is ok.
        ((1 as u8) << (1 << 2)) - 1
    }
}

/// Exersize trait to demonstrate it works.
fn e0109_code_complies_and_works() {
    assert_eq!(<bool as UnsignedBits<bool, 0>>::mask(), 0 != 1);
    assert_eq!(<u8 as UnsignedBits<u8, 0>>::mask(), 0x01_u8);
    assert_eq!(<u8 as UnsignedBits<u8, 1>>::mask(), 0x03_u8);
    assert_eq!(<u8 as UnsignedBits<u8, 2>>::mask(), 0x0f_u8);
    assert_eq!(<u8 as UnsignedBits<u8, 3>>::mask(), 0xff_u8);
}

/// Demonstrate the code compiles, runs, and behaves as expected.
fn main() {
    e0109_code_complies_and_works();
    println!("Hello, world!");
}

#[test]
fn also_works_here() {
    e0109_code_complies_and_works();
}
