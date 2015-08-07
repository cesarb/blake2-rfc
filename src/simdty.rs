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

#[cfg(feature = "simd")]
macro_rules! decl_vec {
    ($($decl:item)*) => {
        $(
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug)]
            #[repr(simd)]
            $decl
        )*
    }
}

#[cfg(not(feature = "simd"))]
macro_rules! decl_vec {
    ($($decl:item)*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[repr(C)]
            $decl
        )*
    }
}

decl_vec!{
    pub struct u32x4(pub u32, pub u32, pub u32, pub u32);
    pub struct u64x4(pub u64, pub u64, pub u64, pub u64);
}

#[cfg(feature = "simd_opt")]
decl_vec!{
    pub struct u16x8(pub u16, pub u16, pub u16, pub u16,
                     pub u16, pub u16, pub u16, pub u16);
    pub struct u32x8(pub u32, pub u32, pub u32, pub u32,
                     pub u32, pub u32, pub u32, pub u32);
}

#[cfg(feature = "simd_opt")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
decl_vec!{
    pub struct u16x16(pub u16, pub u16, pub u16, pub u16,
                      pub u16, pub u16, pub u16, pub u16,
                      pub u16, pub u16, pub u16, pub u16,
                      pub u16, pub u16, pub u16, pub u16);
}

#[cfg(feature = "simd_asm")]
#[cfg(target_arch = "arm")]
decl_vec!{
    pub struct u64x2(pub u64, pub u64);
}
