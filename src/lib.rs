//! A wrapper of UEFI functions.
//!
//! This crate is intended to prvide safe and stable wrappers of UEFI functions. The users of this
//! crate does not have to write any unsafe codes or utilize nightly features.
//!
//! # Usage
//!
//! Specify [`Handle`] and [`SystemTable`] as the parameters of main function.
//!
//! ```rust,no_run
//! # fn main(){}   // See: https://stackoverflow.com/questions/48830213/how-to-doctest-a-function-that-uses-a-trait-from-a-crate
//! // You are supposed to use the `#![no_std]` attribute. In the example, the attribute is
//! // omitted, however, because it requires `eh_personatily` by default.
//!
//! use core::writeln;
//! use core::fmt::Write;
//!
//! #[no_mangle]
//! pub extern "win64" fn efi_main(h: uefi_wrapper::Handle, mut st: uefi_wrapper::SystemTable) -> ! {
//!     let mut con_out = st.con_out();
//!
//!     writeln!(con_out, "hello world");
//!
//!     loop {}
//! }
//! ```

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod error;
pub mod handle;
pub mod protocols;
pub mod result;
pub mod service;
pub mod status;
pub mod system_table;

pub use error::Error;
pub use handle::Handle;
pub use protocols::Protocol;
pub use result::Result;
pub use system_table::SystemTable;
