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

#[cfg(feature = "simd_opt")]
use std::mem::transmute;

pub use simdty::{u32x4, u64x4};

#[cfg(feature = "simd")]
extern "platform-intrinsic" {
    fn simd_add<T>(x: T, y: T) -> T;
    fn simd_shl<T>(x: T, y: T) -> T;
    fn simd_shr<T>(x: T, y: T) -> T;
    fn simd_xor<T>(x: T, y: T) -> T;
}

#[cfg(feature = "simd_opt")]
extern "platform-intrinsic" {
    fn simd_shuffle8<T, Elem>(v: T, w: T,
                             i0: u32, i1: u32, i2: u32, i3: u32,
                             i4: u32, i5: u32, i6: u32, i7: u32) -> T;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn simd_shuffle16<T, Elem>(v: T, w: T,
                                i0: u32,  i1: u32,  i2: u32,  i3: u32,
                                i4: u32,  i5: u32,  i6: u32,  i7: u32,
                                i8: u32,  i9: u32, i10: u32, i11: u32,
                               i12: u32, i13: u32, i14: u32, i15: u32,
                               ) -> T;
}

use std::ops::{Add, BitXor, Shl, Shr};

macro_rules! impl_ops {
    ($vec:ident) => {
        impl Add for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                unsafe { simd_add(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                $vec(self.0.wrapping_add(rhs.0),
                     self.1.wrapping_add(rhs.1),
                     self.2.wrapping_add(rhs.2),
                     self.3.wrapping_add(rhs.3))
            }
        }

        impl BitXor for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                unsafe { simd_xor(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                $vec(self.0 ^ rhs.0,
                     self.1 ^ rhs.1,
                     self.2 ^ rhs.2,
                     self.3 ^ rhs.3)
            }
        }

        impl Shl<$vec> for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shl(self, rhs: Self) -> Self::Output {
                unsafe { simd_shl(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shl(self, rhs: Self) -> Self::Output {
                $vec(self.0 << rhs.0,
                     self.1 << rhs.1,
                     self.2 << rhs.2,
                     self.3 << rhs.3)
            }
        }

        impl Shr<$vec> for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shr(self, rhs: Self) -> Self::Output {
                unsafe { simd_shr(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shr(self, rhs: Self) -> Self::Output {
                $vec(self.0 >> rhs.0,
                     self.1 >> rhs.1,
                     self.2 >> rhs.2,
                     self.3 >> rhs.3)
            }
        }
    }
}

impl_ops!(u32x4);
impl_ops!(u64x4);

pub trait Vector4<T>: Copy {
    fn gather(src: &[T], i0: usize, i1: usize, i2: usize, i3: usize) -> Self;

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

macro_rules! impl_vector4_common {
    ($vec:ident, $word:ident, $bits:expr) => {
        #[inline(always)]
        fn gather(src: &[$word], i0: usize, i1: usize,
                                 i2: usize, i3: usize) -> Self {
            $vec(src[i0], src[i1], src[i2], src[i3])
        }

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

        #[inline(always)]
        fn wrapping_add(self, rhs: Self) -> Self { self + rhs }

        #[cfg(feature = "simd")]
        #[inline(always)]
        fn rotate_right_any(self, n: u32) -> Self {
            let r = n as $word;
            let l = $bits - r;

            (self >> $vec(r, r, r, r)) ^ (self << $vec(l, l, l, l))
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

#[cfg(feature = "simd_opt")]
#[cfg(any(target_arch = "arm", target_arch = "aarch64",
          target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn u32x4_rotate_right_16(vec: u32x4) -> u32x4 {
    use simdty::u16x8;
    unsafe {
        let tmp: u16x8 = transmute(vec);
        transmute(simd_shuffle8::<u16x8, u16>(tmp, tmp,
                                              1, 0,
                                              3, 2,
                                              5, 4,
                                              7, 6))
    }
}

impl Vector4<u32> for u32x4 {
    impl_vector4_common!(u32x4, u32, 32);

    #[cfg(feature = "simd_opt")]
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

#[cfg(feature = "simd_opt")]
#[cfg(any(target_arch = "arm", target_arch = "aarch64",
          target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn u64x4_rotate_right_32(vec: u64x4) -> u64x4 {
    use simdty::u32x8;
    unsafe {
        let tmp: u32x8 = transmute(vec);
        transmute(simd_shuffle8::<u32x8, u32>(tmp, tmp,
                                              1, 0,
                                              3, 2,
                                              5, 4,
                                              7, 6))
    }
}

#[cfg(feature = "simd_opt")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn u64x4_rotate_right_16(vec: u64x4) -> u64x4 {
    use simdty::u16x16;
    unsafe {
        let tmp: u16x16 = transmute(vec);
        transmute(simd_shuffle16::<u16x16, u16>(tmp, tmp,
                                                 1,  2,  3,  0,
                                                 5,  6,  7,  4,
                                                 9, 10, 11,  8,
                                                13, 14, 15, 12))
    }
}

#[cfg(feature = "simd_asm")]
#[cfg(target_arch = "arm")]
use simdty::u64x2;

#[cfg(feature = "simd_asm")]
#[cfg(target_arch = "arm")]
#[inline(always)]
fn vext_u64_u8(a: u64x2, b: u8) -> u64x2 {
    unsafe {
        let result: u64x2;
        asm!("vext.8 ${0:e}, ${1:e}, ${1:e}, $2\nvext.8 ${0:f}, ${1:f}, ${1:f}, $2"
           : "=w" (result)
           : "w" (a), "n" (b));
        result
    }
}

#[cfg(feature = "simd_asm")]
#[cfg(target_arch = "arm")]
#[inline(always)]
fn u64x4_rotate_right_u8(vec: u64x4, n: u8) -> u64x4 {
    let tmp0 = vext_u64_u8(u64x2(vec.0, vec.1), n);
    let tmp1 = vext_u64_u8(u64x2(vec.2, vec.3), n);
    u64x4(tmp0.0, tmp0.1, tmp1.0, tmp1.1)
}

impl Vector4<u64> for u64x4 {
    impl_vector4_common!(u64x4, u64, 64);


    #[cfg(feature = "simd_opt")]
    #[cfg(any(all(target_arch = "arm", not(feature = "simd_asm")),
              target_arch = "aarch64"))]
    #[inline(always)]
    fn rotate_right(self, n: u32) -> Self
    {
        match n {
            32 => u64x4_rotate_right_32(self),
            _ => self.rotate_right_any(n),
        }
    }

    #[cfg(feature = "simd_asm")]
    #[cfg(target_arch = "arm")]
    #[inline(always)]
    fn rotate_right(self, n: u32) -> Self
    {
        match n {
            32 => u64x4_rotate_right_32(self),
            24 => u64x4_rotate_right_u8(self, 3),
            16 => u64x4_rotate_right_u8(self, 2),
            _ => self.rotate_right_any(n),
        }
    }

    #[cfg(feature = "simd_opt")]
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
