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

use simdty;

extern "platform-intrinsic" {
    pub fn simd_add<T>(x: T, y: T) -> T;
    pub fn simd_shl<T>(x: T, y: T) -> T;
    pub fn simd_shr<T>(x: T, y: T) -> T;
    pub fn simd_xor<T>(x: T, y: T) -> T;

    pub fn simd_shuffle2<T, Elem>(v: T, w: T, i0: u32, i1: u32)
        -> simdty::Simd2<Elem>;
    pub fn simd_shuffle4<T, Elem>(v: T, w: T,
                                  i0: u32, i1: u32, i2: u32, i3: u32)
        -> simdty::Simd4<Elem>;
    pub fn simd_shuffle8<T, Elem>(v: T, w: T,
                                  i0: u32, i1: u32, i2: u32, i3: u32,
                                  i4: u32, i5: u32, i6: u32, i7: u32)
        -> simdty::Simd8<Elem>;
    pub fn simd_shuffle16<T, Elem>(v: T, w: T,
                                   i0: u32,  i1: u32,  i2: u32,  i3: u32,
                                   i4: u32,  i5: u32,  i6: u32,  i7: u32,
                                   i8: u32,  i9: u32, i10: u32, i11: u32,
                                  i12: u32, i13: u32, i14: u32, i15: u32)
        -> simdty::Simd16<Elem>;
}
