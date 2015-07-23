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

//! The BLAKE2s hash function.
//!
//! # Examples
//!
//! ```
//! use blake2_rfc::blake2s::{Blake2s, blake2s};
//!
//! // Using the convenience function.
//! let hash = blake2s(32, &[], b"The quick brown fox jumps over the lazy dog");
//!
//! // Using the state context.
//! let mut context = Blake2s::new(32);
//! context.update(b"The quick brown fox jumps over the lazy dog");
//! let hash = context.finalize();
//!
//! // Using the convenience function, with a key.
//! let hash = blake2s(32, b"key", b"The quick brown fox jumps over the lazy dog");
//!
//! // Using the state context, with a key.
//! let mut context = Blake2s::with_key(32, b"key");
//! context.update(b"The quick brown fox jumps over the lazy dog");
//! let hash = context.finalize();
//! ```
//!
//! The returned hash is a `Blake2sResult`, which can be compared with
//! a byte string (the comparison will take constant time), or converted
//! into a byte string.

blake2_impl!(Blake2s, Blake2sResult, blake2s, u32, 32, 16, 12, 8, 7, [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
    0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
]);

blake2_selftest_impl!(Blake2s, blake2s, [
    0x6A, 0x41, 0x1F, 0x08, 0xCE, 0x25, 0xAD, 0xCD,
    0xFB, 0x02, 0xAB, 0xA6, 0x41, 0x45, 0x1C, 0xEC,
    0x53, 0xC5, 0x98, 0xB2, 0x4F, 0x4F, 0xC7, 0x87,
    0xFB, 0xDC, 0x88, 0x79, 0x7F, 0x4C, 0x1D, 0xFE,
], [ 16, 20, 28, 32 ], [ 0,  3,  64, 65, 255, 1024 ]);

blake2_bench_impl!(Blake2s, 32);

#[cfg(test)]
mod tests {
    use std::io::prelude::*;

    extern crate rustc_serialize as serialize;
    use self::serialize::hex::FromHex;

    use blake2::selftest_seq;
    use super::{Blake2s, blake2s};

    #[test]
    fn test_empty() {
        assert_eq!(&blake2s(32, &[], b""),
            &"69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9"
             .from_hex().unwrap()[..]);
    }

    #[test]
    fn selftest() {
        super::selftest();
    }

    #[test]
    fn test_split() {
        let data = selftest_seq(256);

        let mut ctx = Blake2s::new(32);
        ctx.update(&data[..16]);
        ctx.update(&data[16..32]);
        ctx.update(&data[32..224]);
        ctx.update(&data[224..]);

        assert_eq!(&ctx.finalize(), &blake2s(32, &[], &data));
    }

    #[test]
    fn test_write() {
        let data = selftest_seq(65536);

        let mut ctx = Blake2s::new(32);
        ctx.update(&data[..]);

        let mut writer = Blake2s::new(32);
        writer.write_all(&data[..]).unwrap();

        assert_eq!(&writer.finalize(), &ctx.finalize());
    }

    #[test]
    fn test_4g() {
        const ZEROS: [u8; 4096] = [0; 4096];

        let mut state = Blake2s::new(32);
        for _ in 0..1048576 {
            state.update(&ZEROS);
        }
        assert_eq!(&state.finalize(),
            &"2a8e26830310da3ef7f7032b7b1af11b989aba44a3713a22f539f69bd2ce4a87"
             .from_hex().unwrap()[..]);
    }
}
