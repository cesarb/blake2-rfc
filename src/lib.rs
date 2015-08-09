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

//! A pure Rust implementation of BLAKE2 based on the draft RFC.

#![cfg_attr(all(feature = "bench", test), feature(test))]
#![cfg_attr(feature = "simd", feature(platform_intrinsics, simd_basics))]
#![cfg_attr(feature = "simd_opt", feature(cfg_target_feature))]
#![cfg_attr(feature = "simd_asm", feature(asm))]

#[cfg(all(feature = "bench", test))] extern crate test;

extern crate constant_time_eq;

mod as_bytes;
mod bytes;

mod simdty;
#[allow(dead_code)] mod simdint {
    #[cfg(feature = "simd")]
    include!("simdint.rs");
}
mod simd;

#[macro_use]
mod blake2;

pub mod blake2b;
pub mod blake2s;

/// Runs the self-test for both BLAKE2b and BLAKE2s.
pub fn selftest() {
    blake2b::selftest();
    blake2s::selftest();
}
