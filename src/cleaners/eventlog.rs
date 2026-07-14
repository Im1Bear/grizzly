// Nicht selber geschrieben, muss in Zukunft die Windows Crate genauer angucken

use windows::Win32::System::EventLog::{ClearEventLogW, CloseEventLog, OpenEventLogW};
use windows::core::w;

pub fn clean_eventlog() {
    println!("Bereinige Event Logs...");
    let logs = [w!("Application"), w!("System"), w!("Security")];

    for log in logs {
        unsafe {
            match OpenEventLogW(None, log) {
                Ok(handle) => {
                    ClearEventLogW(handle, None).ok();
                    CloseEventLog(handle);
                    println!("Event Log geleert");
                }
                Err(e) => {
                    eprintln!("Fehler beim Öffnen des Event Logs: {}", e);
                }
            }
        }
    }
}
