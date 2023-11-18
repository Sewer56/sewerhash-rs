//! # Some Cool Reloaded Library
//! Here's the crate documentation.

// no_std broken due to multiple crate types defined
// https://github.com/rust-lang/rust/issues/48665
#![cfg_attr(not(test), no_std)]

pub mod implementations {
    pub mod binary {
        pub mod sewerhash_nonvector_64;
    }
}
