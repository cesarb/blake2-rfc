// Copyright (c) 2015 Cesar Eduardo Barros
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#[cfg(feature = "simd")]
use std::mem::transmute;

#[cfg(feature = "simd")]
pub use simdty::{u32x4, u64x4};

#[cfg(not(feature = "simd"))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

#[cfg(not(feature = "simd"))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct u64x4(pub u64, pub u64, pub u64, pub u64);

#[cfg(not(feature = "simd"))]
use std::ops::BitXor;

macro_rules! impl_bitxor {
    ($vec:ident) => {
        #[cfg(not(feature = "simd"))]
        impl BitXor for $vec {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                $vec(self.0 ^ rhs.0,
                     self.1 ^ rhs.1,
                     self.2 ^ rhs.2,
                     self.3 ^ rhs.3)
            }
        }
    }
}

impl_bitxor!(u32x4);
impl_bitxor!(u64x4);

pub trait Vector: Copy {
    fn from_le(self) -> Self;
    fn to_le(self) -> Self;

    fn wrapping_add(self, rhs: Self) -> Self;

    #[inline(always)]
    fn rotate_right(self, n: u32) -> Self { self.rotate_right_any(n) }
    fn rotate_right_any(self, n: u32) -> Self;

    fn shuffle_left_1(self) -> Self;
    fn shuffle_left_2(self) -> Self;
    fn shuffle_left_3(self) -> Self;

    #[inline(always)] fn shuffle_right_1(self) -> Self { self.shuffle_left_3() }
    #[inline(always)] fn shuffle_right_2(self) -> Self { self.shuffle_left_2() }
    #[inline(always)] fn shuffle_right_3(self) -> Self { self.shuffle_left_1() }
}

macro_rules! impl_vector_common {
    ($vec:ident, $word:ident, $bits:expr) => {
        #[cfg(target_endian = "little")]
        #[inline(always)]
        fn from_le(self) -> Self { self }

        #[cfg(not(target_endian = "little"))]
        #[inline(always)]
        fn from_le(self) -> Self {
            $vec($word::from_le(self.0),
                 $word::from_le(self.1),
                 $word::from_le(self.2),
                 $word::from_le(self.3))
        }

        #[cfg(target_endian = "little")]
        #[inline(always)]
        fn to_le(self) -> Self { self }

        #[cfg(not(target_endian = "little"))]
        #[inline(always)]
        fn to_le(self) -> Self {
            $vec(self.0.to_le(),
                 self.1.to_le(),
                 self.2.to_le(),
                 self.3.to_le())
        }

        #[cfg(feature = "simd")]
        #[inline(always)]
        fn wrapping_add(self, rhs: Self) -> Self { self + rhs }

        #[cfg(not(feature = "simd"))]
        #[inline(always)]
        fn wrapping_add(self, rhs: Self) -> Self {
            $vec(self.0.wrapping_add(rhs.0),
                 self.1.wrapping_add(rhs.1),
                 self.2.wrapping_add(rhs.2),
                 self.3.wrapping_add(rhs.3))
        }

        #[cfg(feature = "simd")]
        #[inline(always)]
        fn rotate_right_any(self, n: u32) -> Self {
            let r = n as $word;
            let l = $bits - r;

            (self >> $vec(r, r, r, r)) | (self << $vec(l, l, l, l))
        }

        #[cfg(not(feature = "simd"))]
        #[inline(always)]
        fn rotate_right_any(self, n: u32) -> Self {
            $vec(self.0.rotate_right(n),
                 self.1.rotate_right(n),
                 self.2.rotate_right(n),
                 self.3.rotate_right(n))
        }

        #[inline(always)]
        fn shuffle_left_1(self) -> Self {
            $vec(self.1, self.2, self.3, self.0)
        }

        #[inline(always)]
        fn shuffle_left_2(self) -> Self {
            $vec(self.2, self.3, self.0, self.1)
        }

        #[inline(always)]
        fn shuffle_left_3(self) -> Self {
            $vec(self.3, self.0, self.1, self.2)
        }
    }
}

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "arm", target_arch = "aarch64",
          target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn u32x4_rotate_right_16(vec: u32x4) -> u32x4 {
    use simdty::u16x8;
    unsafe {
        let tmp: u16x8 = transmute(vec);
        transmute(u16x8(tmp.1, tmp.0,
                        tmp.3, tmp.2,
                        tmp.5, tmp.4,
                        tmp.7, tmp.6))
    }
}

impl Vector for u32x4 {
    impl_vector_common!(u32x4, u32, 32);

    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "arm", target_arch = "aarch64",
              target_arch = "x86", target_arch = "x86_64"))]
    #[inline(always)]
    fn rotate_right(self, n: u32) -> Self
    {
        match n {
            16 => u32x4_rotate_right_16(self),
            _ => self.rotate_right_any(n),
        }
    }
}

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "arm", target_arch = "aarch64",
          target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn u64x4_rotate_right_32(vec: u64x4) -> u64x4 {
    use simdty::u32x8;
    unsafe {
        let tmp: u32x8 = transmute(vec);
        transmute(u32x8(tmp.1, tmp.0,
                        tmp.3, tmp.2,
                        tmp.5, tmp.4,
                        tmp.7, tmp.6))
    }
}

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn u64x4_rotate_right_16(vec: u64x4) -> u64x4 {
    use simdty::u16x16;
    unsafe {
        let tmp: u16x16 = transmute(vec);
        transmute(u16x16(tmp.1,  tmp.2,  tmp.3,  tmp.0,
                         tmp.5,  tmp.6,  tmp.7,  tmp.4,
                         tmp.9,  tmp.10, tmp.11, tmp.8,
                         tmp.13, tmp.14, tmp.15, tmp.12))
    }
}

impl Vector for u64x4 {
    impl_vector_common!(u64x4, u64, 64);

    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    #[inline(always)]
    fn rotate_right(self, n: u32) -> Self
    {
        match n {
            32 => u64x4_rotate_right_32(self),
            _ => self.rotate_right_any(n),
        }
    }

    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[inline(always)]
    fn rotate_right(self, n: u32) -> Self
    {
        match n {
            32 => u64x4_rotate_right_32(self),
            16 => u64x4_rotate_right_16(self),
            _ => self.rotate_right_any(n),
        }
    }
}
