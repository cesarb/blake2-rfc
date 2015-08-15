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

#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[cfg(feature = "simd")]
macro_rules! decl_simd {
    ($($decl:item)*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[repr(simd)]
            $decl
        )*
    }
}

#[cfg(not(feature = "simd"))]
macro_rules! decl_simd {
    ($($decl:item)*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[repr(C)]
            $decl
        )*
    }
}

decl_simd! {
    pub struct Simd2<T>(pub T, pub T);
    pub struct Simd4<T>(pub T, pub T, pub T, pub T);
    pub struct Simd8<T>(pub T, pub T, pub T, pub T,
                        pub T, pub T, pub T, pub T);
    pub struct Simd16<T>(pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T);
    pub struct Simd32<T>(pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T);
}

pub type u64x2 = Simd2<u64>;

pub type u32x4 = Simd4<u32>;
pub type u64x4 = Simd4<u64>;

pub type u16x8 = Simd8<u16>;
pub type u32x8 = Simd8<u32>;

pub type u8x16 = Simd16<u8>;
pub type u16x16 = Simd16<u16>;

pub type u8x32 = Simd32<u8>;

impl<T> Simd4<T> {
    #[inline(always)]
    pub fn new(e0: T, e1: T, e2: T, e3: T) -> Simd4<T> {
        Simd4(e0, e1, e2, e3)
    }
}
