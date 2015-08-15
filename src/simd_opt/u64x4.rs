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

use simdty::u64x4;

#[inline(always)]
fn rotate_right_any(vec: u64x4, n: u32) -> u64x4 {
    let r = n as u64;
    let l = 64 - r;

    (vec >> u64x4::new(r, r, r, r)) ^ (vec << u64x4::new(l, l, l, l))
}

#[cfg(feature = "simd_opt")]
#[inline(always)]
pub fn rotate_right_const(vec: u64x4, n: u32) -> u64x4 {
    match n {
        32 => rotate_right_32(vec),
        24 => rotate_right_24(vec),
        16 => rotate_right_16(vec),
        _ => rotate_right_any(vec, n),
    }
}

#[cfg(not(feature = "simd_opt"))]
#[inline(always)]
pub fn rotate_right_const(vec: u64x4, n: u32) -> u64x4 {
    rotate_right_any(vec, n)
}

#[cfg(feature = "simd_opt")]
#[cfg(any(target_feature = "sse2", target_feature = "neon"))]
#[inline(always)]
fn rotate_right_32(vec: u64x4) -> u64x4 {
    use simdint::simd_shuffle8;
    use simdty::u32x8;
    use std::mem::transmute;

    unsafe {
        let tmp: u32x8 = transmute(vec);
        let tmp: u32x8 = simd_shuffle8(tmp, tmp,
                                       [1, 0,
                                        3, 2,
                                        5, 4,
                                        7, 6]);
        transmute(tmp)
    }
}

#[cfg(feature = "simd_opt")]
#[cfg(not(any(target_feature = "sse2", target_feature = "neon")))]
#[inline(always)]
fn rotate_right_32(vec: u64x4) -> u64x4 { rotate_right_any(vec, 32) }

#[cfg(feature = "simd_opt")]
#[cfg(target_feature = "ssse3")]
#[inline(always)]
fn rotate_right_24(vec: u64x4) -> u64x4 {
    use simdint::simd_shuffle32;
    use simdty::u8x32;
    use std::mem::transmute;

    unsafe {
        let tmp: u8x32 = transmute(vec);
        let tmp: u8x32 = simd_shuffle32(tmp, tmp,
                                        [ 3,  4,  5,  6,  7,  0,  1,  2,
                                         11, 12, 13, 14, 15,  8,  9, 10,
                                         19, 20, 21, 22, 23, 16, 17, 18,
                                         27, 28, 29, 30, 31, 24, 25, 26]);
        transmute(tmp)
    }
}

#[cfg(feature = "simd_asm")]
#[cfg(target_feature = "neon")]
#[cfg(target_arch = "arm")]
#[inline(always)]
fn rotate_right_24(vec: u64x4) -> u64x4 {
    rotate_right_u8(vec, 3)
}

#[cfg(feature = "simd_opt")]
#[cfg(not(any(target_feature = "ssse3",
              all(feature = "simd_asm",
                  target_feature = "neon",
                  target_arch = "arm"))))]
#[inline(always)]
fn rotate_right_24(vec: u64x4) -> u64x4 { rotate_right_any(vec, 24) }

#[cfg(feature = "simd_opt")]
#[cfg(target_feature = "sse2")]
#[inline(always)]
fn rotate_right_16(vec: u64x4) -> u64x4 {
    use simdint::simd_shuffle16;
    use simdty::u16x16;
    use std::mem::transmute;

    unsafe {
        let tmp: u16x16 = transmute(vec);
        let tmp: u16x16 = simd_shuffle16(tmp, tmp,
                                         [ 1,  2,  3,  0,
                                           5,  6,  7,  4,
                                           9, 10, 11,  8,
                                          13, 14, 15, 12]);
        transmute(tmp)
    }
}

#[cfg(feature = "simd_asm")]
#[cfg(target_feature = "neon")]
#[cfg(target_arch = "arm")]
#[inline(always)]
fn rotate_right_16(vec: u64x4) -> u64x4 {
    rotate_right_u8(vec, 2)
}

#[cfg(feature = "simd_opt")]
#[cfg(not(any(target_feature = "sse2",
              all(feature = "simd_asm",
                  target_feature = "neon",
                  target_arch = "arm"))))]
#[inline(always)]
fn rotate_right_16(vec: u64x4) -> u64x4 { rotate_right_any(vec, 16) }

#[cfg(feature = "simd_asm")]
#[cfg(target_feature = "neon")]
#[cfg(target_arch = "arm")]
mod simd_asm_neon_arm {
    use simdty::u64x2;

    #[inline(always)]
    fn vext_u64_u8(vec: u64x2, b: u8) -> u64x2 {
        unsafe {
            let result: u64x2;
            asm!("vext.8 ${0:e}, ${1:e}, ${1:e}, $2\nvext.8 ${0:f}, ${1:f}, ${1:f}, $2"
               : "=w" (result)
               : "w" (vec), "n" (b));
            result
        }
    }

    #[inline(always)]
    pub fn rotate_right_u8(vec: u64x4, n: u8) -> u64x4 {
        use simdint::{simd_shuffle2, simd_shuffle4};

        unsafe {
            let tmp0 = vext_u64_u8(simd_shuffle2(vec, vec, [0, 1]), n);
            let tmp1 = vext_u64_u8(simd_shuffle2(vec, vec, [2, 3]), n);
            simd_shuffle4(tmp0, tmp1, [0, 1, 2, 3])
        }
    }
}

#[cfg(feature = "simd_asm")]
#[cfg(target_feature = "neon")]
#[cfg(target_arch = "arm")]
use simd_asm_neon_arm::*;
