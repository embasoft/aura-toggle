#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
use std::io::Error;
use std::io::SeekFrom::Start;
use std::process::Command;
use std::ptr::null_mut;

use winapi::_core::ptr::null;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::LPCSTR;
use winapi::um::winsvc::{ControlService as ControlServiceWINAPI,
                         LPSERVICE_STATUS,
                         OpenSCManagerA,
                         OpenServiceA,
                         QueryServiceStatus,
                         SC_HANDLE,
                         SC_MANAGER_ALL_ACCESS,
                         SERVICE_ALL_ACCESS,
                         SERVICE_CONTROL_CONTINUE,
                         SERVICE_CONTROL_PAUSE,
                         SERVICE_CONTROL_STOP,
                         SERVICE_STATUS,
                         StartServiceA};

use crate::sc_manager::SCManager;
use crate::service::Service;

mod service_status;
mod sc_manager;
mod service;
mod registry;

#[cfg(windows)]
fn main() {
    let scm = SCManager::new("", "", sc_manager::AccessRight::SC_MANAGER_ALL_ACCESS);
    let mut svc = Service::new(scm, "LightingService", service::AccessRight::SERVICE_ALL_ACCESS);
    if svc.is_running() {
        svc.control(service::ControlCode::SERVICE_CONTROL_STOP);
    }

    let program = Command::new("D:\\Spiele\\Tom Clancy's The Division 2\\TheDivision2.exe").output().expect("Failed to start division 2");

    if !svc.is_running() {
        svc.start();
    }
}
