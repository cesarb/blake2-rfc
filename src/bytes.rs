// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of the Rust distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A local copy of the unstable std::slice::bytes module.

use std::ptr;

/// A trait for operations on mutable `[u8]`s.
pub trait MutableByteVector {
    /// Sets all bytes of the receiver to the given value.
    fn set_memory(&mut self, value: u8);
}

impl MutableByteVector for [u8] {
    #[inline]
    fn set_memory(&mut self, value: u8) {
        unsafe { ptr::write_bytes(self.as_mut_ptr(), value, self.len()) };
    }
}

/// Copies data from `src` to `dst`
///
/// Panics if the length of `dst` is less than the length of `src`.
#[inline]
pub fn copy_memory(src: &[u8], dst: &mut [u8]) {
    let len_src = src.len();
    assert!(dst.len() >= len_src);
    // `dst` is unaliasable, so we know statically it doesn't overlap
    // with `src`.
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(),
                                 dst.as_mut_ptr(),
                                 len_src);
    }
}
