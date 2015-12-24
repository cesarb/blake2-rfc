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

//! The BLAKE2b hash function.
//!
//! # Examples
//!
//! ```
//! use blake2_rfc::blake2b::{Blake2b, blake2b};
//!
//! // Using the convenience function.
//! let hash = blake2b(64, &[], b"The quick brown fox jumps over the lazy dog");
//!
//! // Using the state context.
//! let mut context = Blake2b::new(64);
//! context.update(b"The quick brown fox jumps over the lazy dog");
//! let hash = context.finalize();
//!
//! // Using the convenience function, with a key.
//! let hash = blake2b(64, b"key", b"The quick brown fox jumps over the lazy dog");
//!
//! // Using the state context, with a key.
//! let mut context = Blake2b::with_key(64, b"key");
//! context.update(b"The quick brown fox jumps over the lazy dog");
//! let hash = context.finalize();
//! ```
//!
//! The returned hash is a `Blake2bResult`, which can be compared with
//! a byte string (the comparison will take constant time), or converted
//! into a byte string.

blake2_impl!(Blake2b, Blake2bResult, blake2b, u64, u64x4, 64, 32, 24, 16, 63, [
    0x6A09E667F3BCC908, 0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B, 0xA54FF53A5F1D36F1,
    0x510E527FADE682D1, 0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B, 0x5BE0CD19137E2179,
]);

blake2_selftest_impl!(Blake2b, blake2b, [
    0xC2, 0x3A, 0x78, 0x00, 0xD9, 0x81, 0x23, 0xBD,
    0x10, 0xF5, 0x06, 0xC6, 0x1E, 0x29, 0xDA, 0x56,
    0x03, 0xD7, 0x63, 0xB8, 0xBB, 0xAD, 0x2E, 0x73,
    0x7F, 0x5E, 0x76, 0x5A, 0x7B, 0xCC, 0xD4, 0x75,
], [ 20, 32, 48, 64 ], [ 0, 3, 128, 129, 255, 1024 ]);

blake2_bench_impl!(Blake2b, 64);

#[cfg(test)]
mod tests {
    #![allow(result_unwrap_used)]

    use std::io::prelude::*;

    extern crate data_encoding;
    use self::data_encoding::base16;

    use blake2::selftest_seq;
    use super::{Blake2b, blake2b};

    #[test]
    fn test_empty() {
        assert_eq!(&blake2b(64, &[], b""), &base16::decode(
            b"786A02F742015903C6C6FD852552D272912F4740E15847618A86E217F71F5419D25E1031AFEE585313896444934EB04B903A685B1448B755D56F701AFE9BE2CE")
            .unwrap()[..]);
    }

    #[test]
    fn selftest() {
        super::selftest();
    }

    #[test]
    fn test_split() {
        let data = selftest_seq(512);

        let mut ctx = Blake2b::new(64);
        ctx.update(&data[..32]);
        ctx.update(&data[32..64]);
        ctx.update(&data[64..448]);
        ctx.update(&data[448..]);

        assert_eq!(&ctx.finalize(), &blake2b(64, &[], &data));
    }

    #[test]
    fn test_write() {
        let data = selftest_seq(65536);

        let mut ctx = Blake2b::new(64);
        ctx.update(&data[..]);

        let mut writer = Blake2b::new(64);
        writer.write_all(&data[..]).unwrap();

        assert_eq!(&writer.finalize(), &ctx.finalize());
    }

    #[test]
    fn test_4g() {
        const ZEROS: [u8; 4096] = [0; 4096];

        let mut state = Blake2b::new(64);
        for _ in 0..1048576 {
            state.update(&ZEROS);
        }
        assert_eq!(&state.finalize(), &base16::decode(
            b"645572CA5756F9104329ED543735FC11904F0C18C4DF8ADF930F22D07F3094919A519FF34FD240AE3F5D5B4C8042225C109FB951036FDC99E7D2CD0C1D36B267")
            .unwrap()[..]);
    }
}
