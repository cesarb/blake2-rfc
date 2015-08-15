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
macro_rules! transmute_shuffle {
    ($tmp:ident, $shuffle:ident, $vec:expr, $idx:expr) => {
        unsafe {
            use simdty::$tmp;
            use simdint::$shuffle;
            use std::mem::transmute;

            let tmp: $tmp = transmute($vec);
            let tmp: $tmp = $shuffle(tmp, tmp, $idx);
            transmute(tmp)
        }
    }
}

#[cfg(feature = "simd")] pub mod u32x4;
#[cfg(feature = "simd")] pub mod u64x4;

#[cfg(not(feature = "simd"))]
macro_rules! simd_opt {
    ($vec:ident) => {
        pub mod $vec {
            use simdty::$vec;

            #[inline(always)]
            pub fn rotate_right_const(vec: $vec, n: u32) -> $vec {
                $vec::new(vec.0.rotate_right(n),
                          vec.1.rotate_right(n),
                          vec.2.rotate_right(n),
                          vec.3.rotate_right(n))
            }
        }
    }
}

#[cfg(not(feature = "simd"))] simd_opt!(u32x4);
#[cfg(not(feature = "simd"))] simd_opt!(u64x4);
