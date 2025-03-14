use std::{thread, time::Duration, sync::mpsc::Sender};
use windows::{
    core::Error as WindowsError,
    Win32::System::Power::{
        SetThreadExecutionState, ES_DISPLAY_REQUIRED,
        ES_SYSTEM_REQUIRED, EXECUTION_STATE,
        ES_CONTINUOUS
    }
};

#[derive(Clone)]
pub struct KeepAwake {
    previous: EXECUTION_STATE
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        unsafe {
            SetThreadExecutionState(self.previous);
        }
    }
}

impl KeepAwake {
    pub fn new() -> Result<Self, WindowsError> {
        Ok(KeepAwake {
            previous: Default::default()
        })
    }

    pub fn activate(&mut self) -> Result<(), WindowsError> {
        let mut esflags = ES_CONTINUOUS;
        esflags |= ES_DISPLAY_REQUIRED;
        esflags |= ES_SYSTEM_REQUIRED;

        unsafe {
            self.previous = SetThreadExecutionState(esflags);
            if self.previous == EXECUTION_STATE(0) {
                return Err(WindowsError::from_win32());
            }
        }
        
        Ok(())
    }

    pub fn activate_for(&mut self, duration: u64, tx: Sender<()>) {        
        if self.activate().is_ok() {
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(duration));
                let _ = tx.send(());
            });
        }
    }
}