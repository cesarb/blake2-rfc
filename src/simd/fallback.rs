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
pub struct vec4_u32(u32, u32, u32, u32);

#[derive(Clone, Copy, Debug)]
pub struct vec4_u64(u64, u64, u64, u64);

macro_rules! impl_vec4 {
    ($vec:ident, $word:ident) => {
        impl BitXor for $vec {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                $vec(self.0 ^ rhs.0,
                     self.1 ^ rhs.1,
                     self.2 ^ rhs.2,
                     self.3 ^ rhs.3)
            }
        }

        impl $vec {
            #[inline(always)]
            pub fn new(a: $word, b: $word, c: $word, d: $word) -> Self {
                $vec(a, b, c, d)
            }

            #[inline(always)]
            pub fn from_le(self) -> Self {
                $vec($word::from_le(self.0),
                     $word::from_le(self.1),
                     $word::from_le(self.2),
                     $word::from_le(self.3))
            }

            #[inline(always)]
            pub fn to_le(self) -> Self {
                $vec(self.0.to_le(),
                     self.1.to_le(),
                     self.2.to_le(),
                     self.3.to_le())
            }

            #[inline(always)]
            pub fn wrapping_add(self, rhs: Self) -> Self {
                $vec(self.0.wrapping_add(rhs.0),
                     self.1.wrapping_add(rhs.1),
                     self.2.wrapping_add(rhs.2),
                     self.3.wrapping_add(rhs.3))
            }

            #[inline(always)]
            pub fn rotate_right(self, n: u32) -> Self {
                $vec(self.0.rotate_right(n),
                     self.1.rotate_right(n),
                     self.2.rotate_right(n),
                     self.3.rotate_right(n))
            }

            #[inline(always)]
            pub fn shuffle_left_1(self) -> Self {
                $vec(self.1, self.2, self.3, self.0)
            }

            #[inline(always)]
            pub fn shuffle_left_2(self) -> Self {
                $vec(self.2, self.3, self.0, self.1)
            }

            #[inline(always)]
            pub fn shuffle_left_3(self) -> Self {
                $vec(self.3, self.0, self.1, self.2)
            }

            #[inline(always)]
            pub fn shuffle_right_1(self) -> Self { self.shuffle_left_3() }

            #[inline(always)]
            pub fn shuffle_right_2(self) -> Self { self.shuffle_left_2() }

            #[inline(always)]
            pub fn shuffle_right_3(self) -> Self { self.shuffle_left_1() }
        }
    }
}

impl_vec4!(vec4_u32, u32);
impl_vec4!(vec4_u64, u64);
