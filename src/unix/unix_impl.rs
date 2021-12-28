use terminal_size::{Height, Width};

/// TODO: This is awful but it looks like there's no api for this on linux.
///       There seem to be some on OsX though, see:
///       - https://stackoverflow.com/a/1525687/1280961
///       - https://stackoverflow.com/a/35413101/1280961 and https://docs.rs/libproc/latest/libproc/libproc/proc_pid/fn.pidinfo.html
pub fn get_parent_pid(pid: u32) -> Result<u32, Box<dyn std::error::Error>> {
    let mut statfile = std::path::PathBuf::new();
    statfile.push("/proc");
    statfile.push(pid.to_string());
    statfile.push("stat");
    let statfile = statfile;
    let statstr = std::fs::read_to_string(statfile.as_path())?;

    // slice away until including the process name (pos 0 and 1)
    let start = statstr.rfind(')').ok_or("Cannot find ')' in stat string")?;
    let stat_after_processname = &statstr[start + 1..];

    let mut split = stat_after_processname.split_whitespace();
    let _type_str = split.next().ok_or("stat split shorter than 1")?;
    let ppid_str = split.next().ok_or("stat split shorter than 2")?;

    let ppid = ppid_str.parse::<u32>()?;
    return Ok(ppid);
}

pub fn terminal_size_of_fd_string(fd_string: std::ffi::OsString) -> Option<(Width, Height)> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let path = CString::new(fd_string.as_bytes());
    if path.is_err() {
        return None;
    }
    let path = path.unwrap();

    let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDWR | libc::O_NOCTTY) };
    if fd != -1 {
        let ret = terminal_size::terminal_size_using_fd(fd);
        unsafe { libc::close(fd) };
        return ret;
    }
    return None;
}
/// TODO: won't work on all unixes and there doesn't seem to be an api for this
pub fn terminal_size_of_process(pid: u32) -> Option<(Width, Height)> {
    let mut fd = std::path::PathBuf::new();
    fd.push("/proc");
    fd.push(pid.to_string());
    fd.push("fd");
    let fd = fd;
    let mut fd1 = fd.clone();
    fd1.push("1");
    let size = terminal_size_of_fd_string(fd1.into_os_string());
    if !size.is_none() {
        return size;
    }

    let mut fd2 = fd.clone();
    fd2.push("2");
    let size = terminal_size_of_fd_string(fd2.into_os_string());
    if !size.is_none() {
        return size;
    }

    return None;
}

pub fn terminal_size() -> Option<(Width, Height)> {
    let size = terminal_size::terminal_size_using_fd(libc::STDOUT_FILENO);
    if !size.is_none() {
        return size;
    }
    let size = terminal_size::terminal_size_using_fd(libc::STDERR_FILENO);
    if !size.is_none() {
        return size;
    }
    let size = terminal_size::terminal_size_using_fd(libc::STDIN_FILENO);
    if !size.is_none() {
        return size;
    }

    let mut ppid = std::process::id();
    loop {
        let ppid_ret = get_parent_pid(ppid);
        if ppid_ret.is_err() {
            return None;
        } else {
            ppid = ppid_ret.unwrap();
        }

        let size = terminal_size_of_process(ppid);
        if !size.is_none() {
            return size;
        }
    }
}
