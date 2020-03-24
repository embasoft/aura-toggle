use std::ffi::CString;

use winapi::_core::ptr::null_mut;
use winapi::um::winsvc::{ControlService, OpenServiceA, QueryServiceStatus, SC_HANDLE, StartServiceA};
use winapi::um::winsvc::{SERVICE_ALL_ACCESS,
                         SERVICE_CHANGE_CONFIG,
                         SERVICE_ENUMERATE_DEPENDENTS,
                         SERVICE_INTERROGATE,
                         SERVICE_PAUSE_CONTINUE,
                         SERVICE_QUERY_CONFIG,
                         SERVICE_QUERY_STATUS,
                         SERVICE_START,
                         SERVICE_STOP,
                         SERVICE_USER_DEFINED_CONTROL};
use winapi::um::winsvc::{SERVICE_CONTROL_CONTINUE,
                         SERVICE_CONTROL_INTERROGATE,
                         SERVICE_CONTROL_NETBINDADD,
                         SERVICE_CONTROL_NETBINDDISABLE,
                         SERVICE_CONTROL_NETBINDENABLE,
                         SERVICE_CONTROL_NETBINDREMOVE,
                         SERVICE_CONTROL_PARAMCHANGE,
                         SERVICE_CONTROL_PAUSE,
                         SERVICE_CONTROL_STOP};

use crate::sc_manager::SCManager;
use crate::service_status::{ServiceState, ServiceStatus};

pub enum AccessRight {
    SERVICE_ALL_ACCESS = SERVICE_ALL_ACCESS as isize,
    SERVICE_CHANGE_CONFIG = SERVICE_CHANGE_CONFIG as isize,
    SERVICE_ENUMERATE_DEPENDENTS = SERVICE_ENUMERATE_DEPENDENTS as isize,
    SERVICE_INTERROGATE = SERVICE_INTERROGATE as isize,
    SERVICE_PAUSE_CONTINUE = SERVICE_PAUSE_CONTINUE as isize,
    SERVICE_QUERY_CONFIG = SERVICE_QUERY_CONFIG as isize,
    SERVICE_QUERY_STATUS = SERVICE_QUERY_STATUS as isize,
    SERVICE_START = SERVICE_START as isize,
    SERVICE_STOP = SERVICE_STOP as isize,
    SERVICE_USER_DEFINED_CONTROL = SERVICE_USER_DEFINED_CONTROL as isize,
}

pub enum ControlCode {
    SERVICE_CONTROL_CONTINUE = SERVICE_CONTROL_CONTINUE as isize,
    SERVICE_CONTROL_INTERROGATE = SERVICE_CONTROL_INTERROGATE as isize,
    SERVICE_CONTROL_NETBINDADD = SERVICE_CONTROL_NETBINDADD as isize,
    SERVICE_CONTROL_NETBINDDISABLE = SERVICE_CONTROL_NETBINDDISABLE as isize,
    SERVICE_CONTROL_NETBINDENABLE = SERVICE_CONTROL_NETBINDENABLE as isize,
    SERVICE_CONTROL_NETBINDREMOVE = SERVICE_CONTROL_NETBINDREMOVE as isize,
    SERVICE_CONTROL_PARAMCHANGE = SERVICE_CONTROL_PARAMCHANGE as isize,
    SERVICE_CONTROL_PAUSE = SERVICE_CONTROL_PAUSE as isize,
    SERVICE_CONTROL_STOP = SERVICE_CONTROL_STOP as isize,
}

pub struct Service {
    handle: SC_HANDLE,
    sc_manager: SCManager,
    status: ServiceStatus,
}

impl Service {
    pub fn new(sc_manager: SCManager, service_name: &str, desired_access: AccessRight) -> Service {
        let service_name_c = CString::new(service_name).expect("Convert service name to cstring failed.");

        let handle: SC_HANDLE;
        unsafe {
            handle = OpenServiceA(sc_manager.handle(), service_name_c.as_ptr(), desired_access as u32);
        }

        let status = ServiceStatus::new();

        Service {
            handle,
            sc_manager,
            status,
        }
    }

    pub fn control(&mut self, control_code: ControlCode) -> i32 {
        unsafe {
            ControlService(self.handle, control_code as u32, self.status.as_lptr())
        }
    }

    fn refresh_status(&mut self) -> i32 {
        unsafe {
            QueryServiceStatus(self.handle, self.status.as_lptr())
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.refresh_status();
        self.status.current_state() == ServiceState::SERVICE_RUNNING as u32
    }

    pub fn start(&self) -> i32 {
        unsafe {
            StartServiceA(self.handle, 0, null_mut())
        }
    }
}