pub mod dll;
pub mod dll_injection;

use terminal_size::{Height, Width};

/// Returns the size of the terminal of the current process or one of its parents, if available.
/// Iostreams are queried in stdout, stderr and stdin order.
///
/// Return `None` on Error.
pub fn any_terminal_size() -> Option<(Width, Height)> {
    dll::create_dll_if_not_exists_already();

    use std::os::windows::io::RawHandle;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

    let size = terminal_size::terminal_size_using_handle(unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } as RawHandle);
    if !size.is_none() {
        return size;
    }
    let size = terminal_size::terminal_size_using_handle(unsafe { GetStdHandle(STD_ERROR_HANDLE) } as RawHandle);
    if !size.is_none() {
        return size;
    }
    let size = terminal_size::terminal_size_using_handle(unsafe { GetStdHandle(STD_INPUT_HANDLE) } as RawHandle);
    if !size.is_none() {
        return size;
    }
    
    return dll_injection::terminal_size_di();
}

/// Returns the size of the terminal of the given process id or one of its parents, if available.
/// Iostreams are queried in stdout, stderr and stdin order.
///
/// Return `None` on Error.
pub fn any_terminal_size_of_process(process_id: u32) -> Option<(Width, Height)> {
    dll::create_dll_if_not_exists_already();
    return dll_injection::terminal_size_di_of_process(process_id);
}
