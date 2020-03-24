use winapi::um::winsvc::{LPSERVICE_STATUS, SERVICE_STATUS};
use winapi::um::winsvc::{SERVICE_CONTINUE_PENDING,
                         SERVICE_PAUSE_PENDING,
                         SERVICE_PAUSED,
                         SERVICE_RUNNING,
                         SERVICE_START_PENDING,
                         SERVICE_STOP_PENDING,
                         SERVICE_STOPPED};

pub enum ServiceState {
    SERVICE_CONTINUE_PENDING = SERVICE_CONTINUE_PENDING as isize,
    SERVICE_PAUSE_PENDING = SERVICE_PAUSE_PENDING as isize,
    SERVICE_PAUSED = SERVICE_PAUSED as isize,
    SERVICE_RUNNING = SERVICE_RUNNING as isize,
    SERVICE_START_PENDING = SERVICE_START_PENDING as isize,
    SERVICE_STOP_PENDING = SERVICE_STOP_PENDING as isize,
    SERVICE_STOPPED = SERVICE_STOPPED as isize,
}

pub struct ServiceStatus {
    original: SERVICE_STATUS
}

impl ServiceStatus {
    pub fn new() -> ServiceStatus {
        let mut original = SERVICE_STATUS {
            dwServiceType: 0,
            dwCurrentState: 0,
            dwControlsAccepted: 0,
            dwWin32ExitCode: 0,
            dwServiceSpecificExitCode: 0,
            dwCheckPoint: 0,
            dwWaitHint: 0,
        };
        ServiceStatus {
            original
        }
    }

    pub fn as_lptr(&mut self) -> LPSERVICE_STATUS {
        &mut self.original
    }

    pub fn current_state(&self) -> u32 {
        self.original.dwCurrentState
    }
}