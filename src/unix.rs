pub mod unix_impl;

use terminal_size::{Height, Width};

/// Returns the size of the terminal of the current process or one of its parents, if available.
/// Iostreams are queried in stdout, stderr and stdin order.
///
/// Return `None` on Error.
pub fn any_terminal_size() -> Option<(Width, Height)> {
    return unix_impl::terminal_size();
}

/// Returns the size of the terminal of the given process id.
/// Iostreams are queried in stdout, stderr and stdin order.
///
/// Return `None` on Error.
pub fn any_terminal_size_of_process(process_id: u32) -> Option<(Width, Height)> {
    return unix_impl::terminal_size_of_process(process_id);
}
