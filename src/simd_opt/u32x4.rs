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

use simdty::u32x4;

#[inline(always)]
fn rotate_right_any(vec: u32x4, n: u32) -> u32x4 {
    let r = n as u32;
    let l = 32 - r;

    (vec >> u32x4::new(r, r, r, r)) ^ (vec << u32x4::new(l, l, l, l))
}

#[cfg(feature = "simd_opt")]
#[inline(always)]
pub fn rotate_right_const(vec: u32x4, n: u32) -> u32x4 {
    match n {
        16 => rotate_right_16(vec),
         8 => rotate_right_8(vec),
        _ => rotate_right_any(vec, n),
    }
}

#[cfg(not(feature = "simd_opt"))]
#[inline(always)]
pub fn rotate_right_const(vec: u32x4, n: u32) -> u32x4 {
    rotate_right_any(vec, n)
}

#[cfg(feature = "simd_opt")]
#[cfg(any(target_feature = "sse2", target_feature = "neon"))]
#[inline(always)]
fn rotate_right_16(vec: u32x4) -> u32x4 {
    use simdint::simd_shuffle8;
    use simdty::u16x8;
    use std::mem::transmute;

    unsafe {
        let tmp: u16x8 = transmute(vec);
        let tmp: u16x8 = simd_shuffle8(tmp, tmp,
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
fn rotate_right_16(vec: u32x4) -> u32x4 { rotate_right_any(vec, 16) }

#[cfg(feature = "simd_opt")]
#[cfg(target_feature = "ssse3")]
#[inline(always)]
fn rotate_right_8(vec: u32x4) -> u32x4 {
    use simdint::simd_shuffle16;
    use simdty::u8x16;
    use std::mem::transmute;

    unsafe {
        let tmp: u8x16 = transmute(vec);
        let tmp: u8x16 = simd_shuffle16(tmp, tmp,
                                        [ 1,  2,  3,  0,
                                          5,  6,  7,  4,
                                          9, 10, 11,  8,
                                         13, 14, 15, 12]);
        transmute(tmp)
    }
}

#[cfg(feature = "simd_opt")]
#[cfg(not(target_feature = "ssse3"))]
#[inline(always)]
fn rotate_right_8(vec: u32x4) -> u32x4 { rotate_right_any(vec, 8) }
