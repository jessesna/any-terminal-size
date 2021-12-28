//! A simple utility for getting the size of a console of any process or on windows.
//!
//! If the process doesn't have a terminal, all parent processes are searched for one.
//!
//! The windows version has to make use of dll injection because there is no api which
//!   allows to query console infos of foreign processes.
//! The unix version should be considered experimental.
//!
//!  This crate requires a minimum rust version of 1.31.0 (2018-12-06)
//!
//! # Example
//!
//! ```
//! use any_terminal_size::{Width, Height, any_terminal_size};
//!
//! let size = any_terminal_size();
//! if let Some((Width(w), Height(h))) = size {
//!     println!("The terminal size of your process or [transitive] parent process is {} cols wide and {} lines tall", w, h);
//! } else {
//!     println!("Unable to get terminal size");
//! }
//! ```
//!

pub use terminal_size::{Height, Width};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use crate::windows::dll::{
    create_dll, create_dll_if_not_exists_already, remove_dll, remove_dll_if_exists,
};
#[cfg(windows)]
pub use crate::windows::{any_terminal_size, any_terminal_size_of_process};

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
pub use crate::unix::{any_terminal_size, any_terminal_size_of_process};
