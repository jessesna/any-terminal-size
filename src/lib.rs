//! A simple utility for getting the size of a console of any process or on windows.
//!
//! If the process doesn't have a terminal, all parent processes are searched for one.
//!
//! The Linux version is just a passtrough to the terminal_size crate which returns
//! the size of the terminal of the current process.
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
// todo: passthrough; not yet implemented
pub fn any_terminal_size() -> Option<(Width, Height)> {
    let size = terminal_size::terminal_size_using_fd(libc::STDOUT_FILENO);
    if !size.is_none() {
        return size;
    }
    let size = terminal_size::terminal_size_using_fd(libc::STDERR_FILENO);
    if !size.is_none() {
        return size;
    }
    return terminal_size::terminal_size_using_fd(libc::STDIN_FILENO);
}
