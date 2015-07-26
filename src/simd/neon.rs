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

#[derive(Clone, Copy)] #[repr(C)] #[simd] struct i32x4(i32, i32, i32, i32);
#[derive(Clone, Copy)] #[repr(C)] #[simd] struct i64x2(i64, i64);

extern {
    #[link_name = "llvm.arm.neon.vshiftu.v4i32"]
    fn vshiftu_v4i32(a: vec4_u32, b: i32x4) -> vec4_u32;

    #[link_name = "llvm.arm.neon.vshiftu.v2i64"]
    fn vshiftu_v2i64(a: u64x2, b: i64x2) -> u64x2;
}

#[inline(always)]
fn vshlq_n_u32(a: vec4_u32, n: u32) -> vec4_u32 {
    let n_ = n as i32;
    unsafe { vshiftu_v4i32(a, i32x4(n_, n_, n_, n_)) }
}

#[inline(always)]
fn vshrq_n_u32(a: vec4_u32, n: u32) -> vec4_u32 {
    let n_ = -(n as i32);
    unsafe { vshiftu_v4i32(a, i32x4(n_, n_, n_, n_)) }
}

#[inline(always)]
fn vshlq_n_u64(a: u64x2, n: u32) -> u64x2 {
    let n_ = n as i64;
    unsafe { vshiftu_v2i64(a, i64x2(n_, n_)) }
}

#[inline(always)]
fn vshrq_n_u64(a: u64x2, n: u32) ->  u64x2 {
    let n_ = -(n as i64);
    unsafe { vshiftu_v2i64(a, i64x2(n_, n_)) }
}

#[cfg(feature = "simd_asm")]
#[inline(always)]
fn vrev32q_u16(a: vec4_u32) -> vec4_u32 {
    unsafe {
        let result: vec4_u32;
        asm!("vrev32.16 $0, $1"
           : "=w" (result)
           : "w" (a));
        result
    }
}

#[cfg(feature = "simd_asm")]
#[inline(always)]
fn vrev64q_u32(a: u64x2) -> u64x2 {
    unsafe {
        let result: u64x2;
        asm!("vrev64.32 $0, $1"
           : "=w" (result)
           : "w" (a));
        result
    }
}

impl vec4_u32 {
    #[inline(always)]
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        vec4_u32(a, b, c, d)
    }

    #[inline(always)] pub fn from_le(self) -> Self { self }
    #[inline(always)] pub fn to_le(self) -> Self { self }

    #[inline(always)]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self + rhs
    }

    #[inline(always)]
    fn rotate_right_any(self, n: u32) -> Self {
        vshrq_n_u32(self, n) ^ vshlq_n_u32(self, 32 - n)
    }

    #[cfg(feature = "simd_asm")]
    #[inline(always)]
    pub fn rotate_right(self, n: u32) -> Self {
        match n {
            16 => vrev32q_u16(self),
            _ => self.rotate_right_any(n),
        }
    }

    #[cfg(not(feature = "simd_asm"))]
    #[inline(always)]
    pub fn rotate_right(self, n: u32) -> Self {
        self.rotate_right_any(n)
    }

    #[inline(always)]
    pub fn shuffle_left_1(self) -> Self {
        vec4_u32(self.1, self.2, self.3, self.0)
    }

    #[inline(always)]
    pub fn shuffle_left_2(self) -> Self {
        vec4_u32(self.2, self.3, self.0, self.1)
    }

    #[inline(always)]
    pub fn shuffle_left_3(self) -> Self {
        vec4_u32(self.3, self.0, self.1, self.2)
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

    #[inline(always)] pub fn from_le(self) -> Self { self }
    #[inline(always)] pub fn to_le(self) -> Self { self }

    #[inline(always)]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        vec4_u64(self.0 + rhs.0, self.1 + rhs.1)
    }

    #[inline(always)]
    fn rotate_right_any(self, n: u32) -> Self {
        vec4_u64(vshrq_n_u64(self.0, n) ^ vshlq_n_u64(self.0, 64 - n),
                 vshrq_n_u64(self.1, n) ^ vshlq_n_u64(self.1, 64 - n))
    }

    #[cfg(feature = "simd_asm")]
    #[inline(always)]
    pub fn rotate_right(self, n: u32) -> Self {
        match n {
            32 => vec4_u64(vrev64q_u32(self.0), vrev64q_u32(self.1)),
            _ => self.rotate_right_any(n),
        }
    }

    #[cfg(not(feature = "simd_asm"))]
    #[inline(always)]
    pub fn rotate_right(self, n: u32) -> Self {
        self.rotate_right_any(n)
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
