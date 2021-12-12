// see https://www.codeproject.com/Articles/4610/Three-Ways-to-Inject-Your-Code-into-Another-Proces
// see https://stackoverflow.com/questions/39261436/createremotethread-fails-maybe-the-lpbaseaddress-in-the-target-process-is-invali

use terminal_size::{Height, Width};

use std::ffi::CString;
use std::{mem, ptr};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::FARPROC;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::minwindef::TRUE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::HANDLE;
use winapi::um::{errhandlingapi, handleapi, libloaderapi, psapi, synchapi, tlhelp32, winnt};

macro_rules! guard_handle {
    ($x:ident) => {
        ::scopeguard::guard($x, |h| {
            #[allow(unused_unsafe)]
            unsafe {
                ::winapi::um::handleapi::CloseHandle(h);
            }
        })
    };
}
macro_rules! guard_vmem {
    ($process_handle:expr, $address:ident) => {
        ::scopeguard::guard($address, |a| {
            #[allow(unused_unsafe)]
            unsafe {
                ::winapi::um::memoryapi::VirtualFreeEx($process_handle, a, 0, winnt::MEM_RELEASE);
            }
        })
    };
}

fn get_process_by_pid(
    process_id: DWORD,
    processes_snap_handle: winnt::HANDLE,
    process_entry: &mut tlhelp32::PROCESSENTRY32W,
) -> bool {
    if processes_snap_handle == INVALID_HANDLE_VALUE {
        return false;
    }

    let mut pe: tlhelp32::PROCESSENTRY32W = unsafe { std::mem::zeroed() };
    pe.dwSize = std::mem::size_of::<tlhelp32::PROCESSENTRY32W>() as u32;

    if unsafe { tlhelp32::Process32FirstW(processes_snap_handle, &mut pe) } == 0 {
        dbg!(unsafe { errhandlingapi::GetLastError() });
        return false;
    }

    loop {
        if pe.th32ProcessID == process_id {
            *process_entry = pe.clone();
            return true;
        } else if unsafe { tlhelp32::Process32NextW(processes_snap_handle, &mut pe) } == 0 {
            dbg!(unsafe { errhandlingapi::GetLastError() });
            break;
        } else {
        }
    }

    return false;
}

pub fn get_local_proc_address_from_module_handle(
    module_handle: HMODULE,
    proc_name: &str,
) -> Option<FARPROC> {
    let proc_name_str = CString::new(proc_name).unwrap();
    let proc_address =
        unsafe { libloaderapi::GetProcAddress(module_handle, proc_name_str.as_ptr()) };
    if proc_address == ptr::null_mut() {
        return None;
    }

    return Some(proc_address);
}
pub fn get_local_proc_module_and_address(
    wanted_module: &str,
    proc_name: &str,
) -> Option<(HMODULE, FARPROC)> {
    let module_str_u16 = widestring::U16CString::from_str(wanted_module).unwrap();
    let mut module_handle = unsafe { libloaderapi::GetModuleHandleW(module_str_u16.as_ptr()) };
    if module_handle == ptr::null_mut() {
        module_handle = unsafe { winapi::um::libloaderapi::LoadLibraryW(module_str_u16.as_ptr()) };
        if module_handle == ptr::null_mut() {
            return None;
        }
    }

    let proc_address = get_local_proc_address_from_module_handle(module_handle, proc_name);
    if proc_address == None {
        return None;
    }

    return Some((module_handle, proc_address.unwrap()));
}
pub fn get_local_proc_address(wanted_module: &str, proc_name: &str) -> Option<FARPROC> {
    get_local_proc_module_and_address(wanted_module, proc_name).map(|(_, a)| a)
}

pub fn get_remote_proc_address(
    process_handle: HANDLE,
    wanted_module: &str,
    proc_name: &str,
) -> Option<FARPROC> {
    let local_proc_and_module_address = get_local_proc_module_and_address(wanted_module, proc_name);
    assert!(local_proc_and_module_address != None);
    if local_proc_and_module_address == None {
        return None;
    }

    let mut module_handles: [HMODULE; 2048] = [ptr::null_mut(); 2048];
    let mut cb_needed: u32 = 0u32;
    if unsafe {
        psapi::EnumProcessModules(
            process_handle,
            module_handles.as_mut_ptr(),
            (mem::size_of::<HMODULE>() * module_handles.len()) as u32,
            &mut cb_needed as *mut _ as *mut u32,
        )
    } == FALSE
    {
        return None;
    }

    let mut remote_module_handle: HMODULE = ptr::null_mut();
    let nof_modules = cb_needed as usize / mem::size_of::<HMODULE>();
    for i in 0..nof_modules {
        let module_handle = module_handles[i];
        let filename_buffer_len_max = 2049_u32;
        let mut filename_buffer = vec![0; filename_buffer_len_max as usize];
        let filename_buffer_len = unsafe {
            psapi::GetModuleFileNameExW(
                process_handle,
                module_handles[i],
                filename_buffer.as_mut_ptr(),
                filename_buffer_len_max,
            )
        };
        if filename_buffer_len == 0 {
            assert!(false);
            continue;
        }

        let module_name =
            String::from_utf16(&filename_buffer[..filename_buffer_len as usize]).unwrap();
        if module_name == wanted_module
            || std::path::Path::new(&module_name)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                == wanted_module
        {
            remote_module_handle = module_handle;
            break;
        }
    }
    if remote_module_handle.is_null() {
        return None;
    }

    if local_proc_and_module_address.unwrap().0 != remote_module_handle {
        return Some(
            (local_proc_and_module_address.unwrap().1 as u64 + remote_module_handle as u64
                - local_proc_and_module_address.unwrap().0 as u64) as FARPROC,
        );
    } else {
        return Some(local_proc_and_module_address.unwrap().1);
    }
}

pub fn locate_dll(name: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    use std::path::PathBuf;
    let exe_dir = PathBuf::from(std::env::current_exe()?.parent().unwrap());
    let res = exe_dir.join(name);
    Ok(res)
}

//#[allow(non_camel_case_types)]
pub fn get_remote_process_terminal_size(pid: u32) -> Option<(Width, Height)> {
    let injection_dll_name = "any_terminal_size_injection_dll.dll";

    let dll = locate_dll(injection_dll_name);
    if dll.is_err() {
        return None;
    }
    let dll = dll.unwrap();

    #[allow(non_snake_case)]
    let LoadLibrary_address = get_local_proc_address("kernel32.dll", "LoadLibraryW");
    assert!(LoadLibrary_address != None);
    if LoadLibrary_address == None {
        return None;
    }
    #[allow(non_snake_case)]
    let FreeLibrary_address = get_local_proc_address("kernel32.dll", "FreeLibrary");
    assert!(FreeLibrary_address != None);
    if FreeLibrary_address == None {
        return None;
    }

    unsafe {
        use winapi::um::winnt::{
            PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
            PROCESS_VM_READ, PROCESS_VM_WRITE,
        };
        use winapi::um::{memoryapi, processthreadsapi};

        let remote_process_handle = processthreadsapi::OpenProcess(
            PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_READ
                | PROCESS_VM_WRITE
                | PROCESS_VM_OPERATION,
            0,
            pid,
        );
        assert!(!remote_process_handle.is_null());
        if remote_process_handle.is_null() {
            return None;
        }
        let remote_process_handle = guard_handle!(remote_process_handle);

        let dll_u16 = widestring::U16CString::from_os_str(dll.as_os_str()).unwrap();
        let dll_u16_num_bytes = 2 * (dll_u16.len() + 1);
        let dll_name_memory_address = memoryapi::VirtualAllocEx(
            *remote_process_handle,
            ptr::null_mut(),
            dll_u16_num_bytes,
            winnt::MEM_RESERVE | winnt::MEM_COMMIT,
            winnt::PAGE_READWRITE,
        );
        assert!(!dll_name_memory_address.is_null());
        if dll_name_memory_address.is_null() {
            return None;
        }
        let dll_name_memory_address = guard_vmem!(*remote_process_handle, dll_name_memory_address);
        let mut nof_bytes_written = 0;
        let ret = memoryapi::WriteProcessMemory(
            *remote_process_handle,
            *dll_name_memory_address,
            mem::transmute(dll_u16.as_ptr()),
            dll_u16_num_bytes,
            &mut nof_bytes_written,
        );
        assert!(ret == TRUE);
        assert!(dll_u16_num_bytes == nof_bytes_written);
        if ret == FALSE || dll_u16_num_bytes != nof_bytes_written {
            return None;
        }

        let remote_thread_handle = processthreadsapi::CreateRemoteThread(
            *remote_process_handle,
            ptr::null_mut(),
            0,
            mem::transmute(LoadLibrary_address.unwrap()),
            *dll_name_memory_address,
            0,
            ptr::null_mut(),
        );
        assert!(!remote_thread_handle.is_null());
        let remote_thread_handle = guard_handle!(remote_thread_handle);

        synchapi::WaitForSingleObject(*remote_thread_handle, INFINITE);

        let _unload_dll_guard = ::scopeguard::guard((), |_| {
            let remote_thread_handle = processthreadsapi::CreateRemoteThread(
                *remote_process_handle,
                ptr::null_mut(),
                0,
                mem::transmute(FreeLibrary_address.unwrap()),
                *dll_name_memory_address,
                0,
                ptr::null_mut(),
            );
            if !remote_thread_handle.is_null() {
                handleapi::CloseHandle(remote_thread_handle);
            }
        });

        let mut injected_dll_base_address = 0_u32;
        let got_exit_code = processthreadsapi::GetExitCodeThread(
            *remote_thread_handle,
            &mut injected_dll_base_address as _,
        );
        assert!(got_exit_code == TRUE);
        if got_exit_code == FALSE || injected_dll_base_address == 0 {
            return None;
        }

        let remote_proc_address = get_remote_proc_address(
            *remote_process_handle,
            injection_dll_name,
            "terminal_size_thread_proc",
        );
        if remote_proc_address == None {
            return None;
        }

        const SIZE_OF_EXCHANGE_BUFFER: usize = 2 * std::mem::size_of::<u16>();
        let exchange_buffer_address = memoryapi::VirtualAllocEx(
            *remote_process_handle,
            ptr::null_mut(),
            SIZE_OF_EXCHANGE_BUFFER as SIZE_T,
            winnt::MEM_RESERVE | winnt::MEM_COMMIT,
            winnt::PAGE_READWRITE,
        );
        assert!(!exchange_buffer_address.is_null());
        if exchange_buffer_address.is_null() {
            return None;
        }
        let exchange_buffer_address = guard_vmem!(*remote_process_handle, exchange_buffer_address);

        let remote_thread_handle = processthreadsapi::CreateRemoteThread(
            *remote_process_handle,
            ptr::null_mut(),
            0,
            mem::transmute(remote_proc_address.unwrap()),
            *exchange_buffer_address,
            0,
            ptr::null_mut(),
        );
        assert!(!remote_thread_handle.is_null());
        let remote_thread_handle = guard_handle!(remote_thread_handle);

        synchapi::WaitForSingleObject(*remote_thread_handle, INFINITE);

        let /*todo? mut*/ read_buffer: [u8; SIZE_OF_EXCHANGE_BUFFER] = [0; SIZE_OF_EXCHANGE_BUFFER];
        let mut nof_bytes_read = 0;
        let ret = memoryapi::ReadProcessMemory(
            *remote_process_handle,
            mem::transmute(*exchange_buffer_address),
            mem::transmute(read_buffer.as_ptr()),
            SIZE_OF_EXCHANGE_BUFFER,
            &mut nof_bytes_read,
        );
        assert!(ret == TRUE);
        assert!(SIZE_OF_EXCHANGE_BUFFER == nof_bytes_read);
        if ret == FALSE || SIZE_OF_EXCHANGE_BUFFER != nof_bytes_read {
            return None;
        }

        let mut w: u16 = 0;
        let mut h: u16 = 0;
        std::ptr::copy_nonoverlapping(
            read_buffer.as_ptr(),
            &mut w as *mut u16 as *mut u8,
            std::mem::size_of_val(&w),
        );
        std::ptr::copy_nonoverlapping(
            (read_buffer.as_ptr() as u64 + 2) as *const u8,
            &mut h as *mut u16 as *mut u8,
            std::mem::size_of_val(&h),
        );

        if w == 0 && h == 0 {
            return None;
        } else {
            return Some((Width(w), Height(h)));
        }
    }
}

pub fn terminal_size_di_of_process(pid: u32) -> Option<(Width, Height)> {
    let processes_snap_handle =
        unsafe { tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0) };
    if processes_snap_handle == INVALID_HANDLE_VALUE {
        return None;
    }
    let processes_snap_handle = guard_handle!(processes_snap_handle);

    let mut process: tlhelp32::PROCESSENTRY32W = unsafe { std::mem::zeroed() };
    if !get_process_by_pid(pid, *processes_snap_handle, &mut process) {
        return None;
    }

    loop {
        if !get_process_by_pid(
            process.th32ParentProcessID,
            *processes_snap_handle,
            &mut process,
        ) {
            return None;
        }
        let size = get_remote_process_terminal_size(process.th32ProcessID);
        if !size.is_none() {
            return size;
        }
    }
}

pub fn terminal_size_di() -> Option<(Width, Height)> {
    let cpid = unsafe { winapi::um::processthreadsapi::GetCurrentProcessId() };
    return terminal_size_di_of_process(cpid);
}
