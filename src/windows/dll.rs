//! Doesn't seem like there's a cargo way of packaging cdylibs.
//! So instead we put a binary representation in the static lib and create it ad hoc on first use.
//! Unfortunately, there's no way to uninstall the dll's with cargo uninstall directly.
//! Sorry for all this. As soon as there's a built in way it'll be changed.

pub mod dll_contents;

pub fn dll_path() -> std::path::PathBuf {
    let exe_path = std::env::current_exe();
    if exe_path.is_err() {
        return std::path::PathBuf::new();
    }
    let mut dll_file_path = exe_path.unwrap();
    dll_file_path.pop();
    dll_file_path.push("any_terminal_size_injection_dll.dll");
    return dll_file_path;
}

pub fn create_dll() -> std::io::Result<()> {
    let dll_bytes = dll_contents::dll_bytes();

    let mut dll_file =
        std::fs::File::create(dll_path().into_os_string().into_string().unwrap()).unwrap();
    use std::io::prelude::*;
    return dll_file.write_all(dll_bytes);
}
pub fn create_dll_if_not_exists_already() -> bool {
    if create_dll().is_ok() {
        return true;
    } else if dll_path().exists() {
        return true;
    } else {
        return false;
    }
}

pub fn remove_dll() -> std::io::Result<()> {
    return std::fs::remove_file(dll_path());
}
pub fn remove_dll_if_exists() -> bool {
    if !dll_path().exists() {
        return true;
    } else if remove_dll().is_ok() {
        return true;
    } else {
        return false;
    }
}
