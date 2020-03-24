#[cfg(windows)]
extern crate winapi;

mod service_status;
mod sc_manager;
mod service;

use std::io::Error;
use std::ptr::null_mut;
use std::ffi::CString;
use winapi::um::winnt::LPCSTR;
use winapi::shared::minwindef::{DWORD, BOOL};
use winapi::um::winsvc::{OpenSCManagerA,
                         OpenServiceA,
                         ControlService as ControlServiceWINAPI,
                         SC_MANAGER_ALL_ACCESS,
                         SC_HANDLE,
                         SERVICE_ALL_ACCESS,
                         SERVICE_STATUS,
                         LPSERVICE_STATUS,
                         SERVICE_CONTROL_STOP,
                         SERVICE_CONTROL_PAUSE,
                         SERVICE_CONTROL_CONTINUE,
                         QueryServiceStatus,
                         StartServiceA};
use winapi::_core::ptr::null;
use winapi::um::errhandlingapi::GetLastError;
use std::io::SeekFrom::Start;
use crate::sc_manager::SCManager;
use crate::service::Service;

#[cfg(windows)]
// fn print_message(msg: &str) -> Result<i32, Error> {
//     use std::ffi::OsStr;
//     use std::iter::once;
//     use std::os::windows::ffi::OsStrExt;
//
//     let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
//     let ret = unsafe {
//         MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_YESNO | MB_ICONASTERISK)
//     };
//     if ret == 0 { Err(Error::last_os_error()) }
//     else { Ok(ret) }
// }

fn main() {
    let scm = SCManager::new("", "", sc_manager::AccessRight::SC_MANAGER_ALL_ACCESS);
    let mut svc = Service::new(scm, "LightingService", service::AccessRight::SERVICE_ALL_ACCESS);
    if svc.is_running() {
        svc.control(service::ControlCode::SERVICE_CONTROL_STOP);
    } else {
        svc.start();
    }



    unsafe {
        println!("error {:?}", GetLastError());
    }
    // match print_message("Hello, world!") {
    //     Ok(IDYES) => println!("{}", "Yes"),
    //     Ok(IDNO) => println!("{}", "No"),
    //     Ok(_) => println!("Unknown event"),
    //     Err(_) => println!("Error occured! :(")
    // }
}
