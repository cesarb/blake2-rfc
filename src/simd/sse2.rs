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

use std::mem::transmute;
use std::ops::BitXor;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[simd]
pub struct vec4_u32(u32, u32, u32, u32);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[simd]
struct u64x2(u64, u64);

#[derive(Clone, Copy, Debug)]
pub struct vec4_u64(u64x2, u64x2);

#[derive(Clone, Copy)]
#[repr(C)]
#[simd]
struct u16x8(u16, u16, u16, u16, u16, u16, u16, u16);

extern {
    #[link_name = "llvm.x86.sse2.pslli.d"]
    fn pslli_d(a: vec4_u32, b: u32) -> vec4_u32;

    #[link_name = "llvm.x86.sse2.pslli.q"]
    fn pslli_q(a: u64x2, b: u32) -> u64x2;

    #[link_name = "llvm.x86.sse2.psrli.d"]
    fn psrli_d(a: vec4_u32, b: u32) -> vec4_u32;

    #[link_name = "llvm.x86.sse2.psrli.q"]
    fn psrli_q(a: u64x2, b: u32) -> u64x2;

    #[link_name = "llvm.x86.sse2.pshuf.d"]
    fn pshuf_d(a: vec4_u32, b: u8) -> vec4_u32;

    #[link_name = "llvm.x86.sse2.pshufl.w"]
    fn pshufl_w(a: u16x8, b: u8) -> u16x8;

    #[link_name = "llvm.x86.sse2.pshufh.w"]
    fn pshufh_w(a: u16x8, b: u8) -> u16x8;
}

impl vec4_u32 {
    #[inline(always)]
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        vec4_u32(a, b, c, d)
    }

    #[inline(always)]
    pub fn from_le(self) -> Self {
        // SSE2 is always little-endian
        self
    }

    #[inline(always)]
    pub fn to_le(self) -> Self {
        // SSE2 is always little-endian
        self
    }

    #[inline(always)]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self + rhs
    }

    #[inline(always)]
    fn rotate_right_16(self) -> Self {
        unsafe {
            let tmp = pshufl_w(transmute(self), 0b10_11_00_01);
            transmute(pshufh_w(tmp,             0b10_11_00_01))
        }
    }

    #[inline(always)]
    fn rotate_right_any(self, n: u32) -> Self {
        unsafe {
            psrli_d(self, n) ^ pslli_d(self, 32 - n)
        }
    }

    #[inline(always)]
    pub fn rotate_right(self, n: u32) -> Self {
        match n {
            16 => self.rotate_right_16(),
            _ => self.rotate_right_any(n),
        }
    }

    #[inline(always)]
    pub fn shuffle_left_1(self) -> Self {
        unsafe {
            pshuf_d(self, 0b00_11_10_01)
        }
    }

    #[inline(always)]
    pub fn shuffle_left_2(self) -> Self {
        unsafe {
            pshuf_d(self, 0b01_00_11_10)
        }
    }

    #[inline(always)]
    pub fn shuffle_left_3(self) -> Self {
        unsafe {
            pshuf_d(self, 0b10_01_00_11)
        }
    }

    #[inline(always)]
    pub fn shuffle_right_1(self) -> Self { self.shuffle_left_3() }

    #[inline(always)]
    pub fn shuffle_right_2(self) -> Self { self.shuffle_left_2() }

    #[inline(always)]
    pub fn shuffle_right_3(self) -> Self { self.shuffle_left_1() }
}

impl BitXor for vec4_u64 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        vec4_u64(self.0 ^ rhs.0,
                 self.1 ^ rhs.1)
    }
}

impl vec4_u64 {
    #[inline(always)]
    pub fn new(a: u64, b: u64, c: u64, d: u64) -> Self {
        vec4_u64(u64x2(a, b), u64x2(c, d))
    }

    #[inline(always)]
    pub fn from_le(self) -> Self {
        // SSE2 is always little-endian
        self
    }

    #[inline(always)]
    pub fn to_le(self) -> Self {
        // SSE2 is always little-endian
        self
    }

    #[inline(always)]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        vec4_u64(self.0 + rhs.0, self.1 + rhs.1)
    }

    #[inline(always)]
    fn rotate_right_32(v: u64x2) -> u64x2 {
        unsafe { transmute(pshuf_d(transmute(v), 0b10_11_00_01)) }
    }

    #[inline(always)]
    fn rotate_right_16(v: u64x2) -> u64x2 {
        unsafe {
            let tmp = pshufl_w(transmute(v), 0b00_11_10_01);
            transmute(pshufh_w(tmp,          0b00_11_10_01))
        }
    }

    #[inline(always)]
    fn rotate_right_any(self, n: u32) -> Self {
        unsafe {
            vec4_u64(psrli_q(self.0, n) ^ pslli_q(self.0, 64 - n),
                     psrli_q(self.1, n) ^ pslli_q(self.1, 64 - n))
        }
    }

    #[inline(always)]
    pub fn rotate_right(self, n: u32) -> Self {
        match n {
            32 => vec4_u64(vec4_u64::rotate_right_32(self.0),
                           vec4_u64::rotate_right_32(self.1)),
            16 => vec4_u64(vec4_u64::rotate_right_16(self.0),
                           vec4_u64::rotate_right_16(self.1)),
            _ => self.rotate_right_any(n),
        }
    }

    #[inline(always)]
    pub fn shuffle_left_1(self) -> Self {
        vec4_u64(u64x2((self.0).1, (self.1).0), u64x2((self.1).1, (self.0).0))
    }

    #[inline(always)]
    pub fn shuffle_left_2(self) -> Self {
        vec4_u64(self.1, self.0)
    }

    #[inline(always)]
    pub fn shuffle_left_3(self) -> Self {
        vec4_u64(u64x2((self.1).1, (self.0).0), u64x2((self.0).1, (self.1).0))
    }

    #[inline(always)]
    pub fn shuffle_right_1(self) -> Self { self.shuffle_left_3() }

    #[inline(always)]
    pub fn shuffle_right_2(self) -> Self { self.shuffle_left_2() }

    #[inline(always)]
    pub fn shuffle_right_3(self) -> Self { self.shuffle_left_1() }
}
