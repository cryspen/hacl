//! Bignum
//!
//! This module implements friendlier bignum for 4096 bit bignums
//! 
//! It safely (one hopes) wraps the unsafe Hacl_Bignum operations and provides
//! a struct (type) Bignum that should conceal the nasty pointers to mutable data.

use hacl_rust_sys::*;
use std::{slice};
use libc::{free};


const BN_BITSIZE: usize = 4096;

// TODO: We need a feature flag for 32 v 64 bit systems
// This is for building for 64bit systems.
const BN_SLICE_LENGTH: usize = BN_BITSIZE / 64;
// This is for building for 32bit systems.
// const BN_SLICE_LENGTH: usize = BN_BITSIZE / 32;



#[derive(Debug)]
/// Errors for Bignum operations
pub enum Error {
    DeconversionError,
    ConversionError,
    AllocationError,
}

/// HaclBnType is used in unsafe operations

type HaclBnType = *mut u64;

#[derive(Debug)]
pub struct Bignum<'a> {
    // There does not appear to be a way to get the size of a hacl_Bignum other
    // than to use the hacl functions for turning one into a byte array.
    // So we will use a byte array as our primary internal representation
    bn: [u8; BN_BITSIZE / 8],

    // hacl_bn is a slice that is the Rust-friendly version of their `*mut u64`
    // It is designed to be converted into what the FFI wants without having to
    // go through Hacl_Bignum4096_new_bn_from_bytes_be each time.
    hacl_bn: &'a [u64]
}


// We will really want From<whatever-we-use-in-core-for-byte-arrays>
impl TryFrom <&[u8]> for Bignum<'_> {
    type Error = Error;
    fn try_from(be_vec: &[u8]) -> Result<Bignum<'static>, Error> {
        let length:u32 = be_vec.len() as u32;
        let bn: [u8; 512];
        let hacl_bn: &[u64];
        
        unsafe {
            let hacl_raw_bn: HaclBnType = Hacl_Bignum4096_new_bn_from_bytes_be(
                length,
                be_vec.as_mut_ptr()
            );
            if hacl_raw_bn.is_null() {
                return Err(Error::AllocationError)
            }
            hacl_bn = slice::from_raw_parts(hacl_raw_bn, 64 as usize);
            Hacl_Bignum4096_bn_to_bytes_be(hacl_raw_bn, bn.as_mut_ptr())
            libc::free(hacl_raw_bn)
        };

        Ok(Self {bn, hacl_bn})
    }
}

impl Bignum<'_> {
    pub fn to_vec8(&self) ->Vec<u8> {
        self.bn.to_vec()
    }
}
