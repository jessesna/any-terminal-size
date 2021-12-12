use terminal_size::{terminal_size, Height, Width};

#[no_mangle]
pub extern "C" fn terminal_size_thread_proc(target_memory: *mut std::ffi::c_void) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        unsafe {
            libc::memcpy(
                target_memory,
                &w as *const u16 as *const libc::c_void,
                std::mem::size_of::<u16>(),
            );
            libc::memcpy(
                target_memory.offset(std::mem::size_of::<u16>() as isize),
                &h as *const u16 as *const libc::c_void,
                std::mem::size_of::<u16>(),
            );
        };
    } else {
        unsafe { libc::memset(target_memory, 0, 2 * std::mem::size_of::<u16>()) };
    }
}
