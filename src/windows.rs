pub mod dll;
pub mod dll_injection;

use terminal_size::{Height, Width};

/// Returns the size of the terminal of the current process or one of its parents, if available.
///
/// Return `None` on Error.
pub fn any_terminal_size() -> Option<(Width, Height)> {
    dll::create_dll_if_not_exists_already();

    let size = terminal_size::terminal_size();
    if !size.is_none() {
        return size;
    } else {
        return dll_injection::terminal_size_di();
    }
}

/// Returns the size of the terminal of the given process id or one of its parents, if available.
///
/// Return `None` on Error.
pub fn any_terminal_size_of_process(process_id: u32) -> Option<(Width, Height)> {
    dll::create_dll_if_not_exists_already();
    return dll_injection::terminal_size_di_of_process(process_id);
}
