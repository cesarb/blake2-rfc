// Copyright 2015 blake2-rfc Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(unused_macros)]

#[cfg(feature = "simd")]
macro_rules! transmute_shuffle {
    ($tmp:ident, $shuffle:ident, $vec:expr, $idx:expr) => {
        unsafe {
            use coresimd::simd::$tmp;
            use simdint::$shuffle;
            use core::mem::transmute;

            let tmp_i: $tmp = transmute($vec);
            let tmp_o: $tmp = $shuffle(tmp_i, tmp_i, $idx);
            transmute(tmp_o)
        }
    }
}

#[cfg(feature = "simd")]
macro_rules! simd_opt {
    ($vec:ident) => {
        pub mod $vec;
    }
}

#[cfg(not(feature = "simd"))]
macro_rules! simd_opt {
    ($vec:ident) => {
        pub mod $vec {
            use simd::$vec;

            #[inline]
            pub fn rotate_right_const(vec: $vec, n: u32) -> $vec {
                $vec::new(vec.0.rotate_right(n),
                          vec.1.rotate_right(n),
                          vec.2.rotate_right(n),
                          vec.3.rotate_right(n))
            }
        }
    }
}

simd_opt!(u32x4);
simd_opt!(u64x4);
