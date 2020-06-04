//! Crate ruma_appservice_api contains serializable types for the requests and responses for each
//! endpoint in the [Matrix](https://matrix.org/) application service API specification. These
//! types can be shared by application service and server code.

#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
// Since we support Rust 1.34.2, we can't apply this suggestion yet
#![allow(clippy::use_self)]

pub mod v1;