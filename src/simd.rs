// Copyright 2015 blake2-rfc Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use as_bytes::Safe;
use simd_opt;

#[cfg(feature = "simd")]
pub use coresimd::simd::{u32x4, u64x4};

#[cfg(not(feature = "simd"))]
pub use simd::fallback::{u32x4, u64x4};

unsafe impl Safe for u32x4 {}
unsafe impl Safe for u64x4 {}

#[cfg(not(feature = "simd"))]
mod fallback {
    use core::ops::{Add, BitXor, BitXorAssign};

    macro_rules! impl_vec {
        ($vec:ident, $elem:ident) => {
            #[derive(Clone, Copy, Debug)]
            #[repr(C)]
            pub struct $vec(pub $elem, pub $elem, pub $elem, pub $elem);

            impl $vec {
                #[inline]
                pub fn new(x0: $elem, x1: $elem, x2: $elem, x3: $elem) -> Self {
                    $vec(x0, x1, x2, x3)
                }
            }

            impl Add for $vec {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    $vec::new(self.0.wrapping_add(rhs.0),
                              self.1.wrapping_add(rhs.1),
                              self.2.wrapping_add(rhs.2),
                              self.3.wrapping_add(rhs.3))
                }
            }

            impl BitXor for $vec {
                type Output = Self;

                #[inline]
                fn bitxor(self, rhs: Self) -> Self::Output {
                    $vec::new(self.0 ^ rhs.0,
                              self.1 ^ rhs.1,
                              self.2 ^ rhs.2,
                              self.3 ^ rhs.3)
                }
            }

            impl BitXorAssign for $vec {
                #[inline]
                fn bitxor_assign(&mut self, rhs: Self) {
                    self.0 ^= rhs.0;
                    self.1 ^= rhs.1;
                    self.2 ^= rhs.2;
                    self.3 ^= rhs.3;
                }
            }
        }
    }

    impl_vec!(u32x4, u32);
    impl_vec!(u64x4, u64);
}

pub trait Vector4<T>: Copy {
    fn gather(src: &[T], i0: usize, i1: usize, i2: usize, i3: usize) -> Self;

    fn from_le(self) -> Self;
    fn to_le(self) -> Self;

    fn wrapping_add(self, rhs: Self) -> Self;

    fn rotate_right_const(self, n: u32) -> Self;

    fn shuffle_left_1(self) -> Self;
    fn shuffle_left_2(self) -> Self;
    fn shuffle_left_3(self) -> Self;

    #[inline] fn shuffle_right_1(self) -> Self { self.shuffle_left_3() }
    #[inline] fn shuffle_right_2(self) -> Self { self.shuffle_left_2() }
    #[inline] fn shuffle_right_3(self) -> Self { self.shuffle_left_1() }
}

macro_rules! impl_vector4 {
    ($vec:ident, $word:ident) => {
        impl Vector4<$word> for $vec {
            #[inline]
            fn gather(src: &[$word], i0: usize, i1: usize,
                                     i2: usize, i3: usize) -> Self {
                $vec::new(src[i0], src[i1], src[i2], src[i3])
            }

            #[cfg(target_endian = "little")]
            #[inline]
            fn from_le(self) -> Self {
                self
            }

            #[cfg(not(target_endian = "little"))]
            #[inline]
            fn from_le(self) -> Self {
                $vec::new($word::from_le(self.0),
                          $word::from_le(self.1),
                          $word::from_le(self.2),
                          $word::from_le(self.3))
            }

            #[cfg(target_endian = "little")]
            #[inline]
            fn to_le(self) -> Self {
                self
            }

            #[cfg(not(target_endian = "little"))]
            #[inline]
            fn to_le(self) -> Self {
                $vec::new(self.0.to_le(),
                          self.1.to_le(),
                          self.2.to_le(),
                          self.3.to_le())
            }

            #[inline(always)]
            fn wrapping_add(self, rhs: Self) -> Self {
                self + rhs
            }

            #[inline(always)]
            fn rotate_right_const(self, n: u32) -> Self {
                simd_opt::$vec::rotate_right_const(self, n)
            }

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shuffle_left_1(self) -> Self {
                use simdint::simd_shuffle4;
                unsafe { simd_shuffle4(self, self, [1, 2, 3, 0]) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shuffle_left_1(self) -> Self {
                $vec::new(self.1, self.2, self.3, self.0)
            }

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shuffle_left_2(self) -> Self {
                use simdint::simd_shuffle4;
                unsafe { simd_shuffle4(self, self, [2, 3, 0, 1]) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shuffle_left_2(self) -> Self {
                $vec::new(self.2, self.3, self.0, self.1)
            }

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shuffle_left_3(self) -> Self {
                use simdint::simd_shuffle4;
                unsafe { simd_shuffle4(self, self, [3, 0, 1, 2]) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shuffle_left_3(self) -> Self {
                $vec::new(self.3, self.0, self.1, self.2)
            }
        }
    }
}

impl_vector4!(u32x4, u32);
impl_vector4!(u64x4, u64);
