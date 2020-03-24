use std::ffi::CString;

use winapi::_core::ptr::null_mut;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::winnt::LPCSTR;
use winapi::um::winsvc::{OpenSCManagerA, SC_HANDLE};
use winapi::um::winsvc::{SC_MANAGER_ALL_ACCESS,
                         SC_MANAGER_CONNECT,
                         SC_MANAGER_CREATE_SERVICE,
                         SC_MANAGER_ENUMERATE_SERVICE,
                         SC_MANAGER_LOCK,
                         SC_MANAGER_MODIFY_BOOT_CONFIG,
                         SC_MANAGER_QUERY_LOCK_STATUS};

pub enum AccessRight {
    SC_MANAGER_ALL_ACCESS = SC_MANAGER_ALL_ACCESS as isize,
    SC_MANAGER_CONNECT = SC_MANAGER_CONNECT as isize,
    SC_MANAGER_CREATE_SERVICE = SC_MANAGER_CREATE_SERVICE as isize,
    SC_MANAGER_ENUMERATE_SERVICE = SC_MANAGER_ENUMERATE_SERVICE as isize,
    SC_MANAGER_LOCK = SC_MANAGER_LOCK as isize,
    SC_MANAGER_MODIFY_BOOT_CONFIG = SC_MANAGER_MODIFY_BOOT_CONFIG as isize,
    SC_MANAGER_QUERY_LOCK_STATUS = SC_MANAGER_QUERY_LOCK_STATUS as isize,
}

pub struct SCManager {
    handle: SC_HANDLE
}

impl SCManager {
    pub fn new(machine_name: &str, database_name: &str, desired_access: AccessRight) -> SCManager {
        let machine_name_lpcstr = match machine_name {
            "" => null_mut(),
            _ => CString::new(machine_name).expect("Convert machine name to cstring failed.").as_ptr()
        };
        let database_name_lpcstr = match database_name {
            "" => null_mut(),
            _ => CString::new(database_name).expect("Convert database to cstring failed").as_ptr()
        };

        let handle: SC_HANDLE;
        unsafe {
            handle = OpenSCManagerA(machine_name_lpcstr, database_name_lpcstr, desired_access as u32);
        }
        SCManager {
            handle: handle
        }
    }

    pub fn handle(&self) -> SC_HANDLE {
        self.handle
    }
}