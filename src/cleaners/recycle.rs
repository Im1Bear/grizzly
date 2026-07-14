use windows::Win32::UI::Shell::{
    SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI, SHERB_NOSOUND, SHEmptyRecycleBinW,
};
use windows::core::PCWSTR;

pub fn clean_recycle() {
    // Flag-Kombination bitweise verknüpfen (0000_0111)
    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;

    unsafe {
        // HWND::default() entspricht NULL (kein übergeordnetes Fenster)
        // PCWSTR::null() bedeutet, dass alle Papierkörbe auf allen Laufwerken geleert werden
        let result = SHEmptyRecycleBinW(None, PCWSTR::null(), flags);

        match result {
            Ok(_) => println!("Der Papierkorb wurde erfolgreich geleert."),
            Err(e) => eprintln!("Fehler beim Leeren des Papierkorbs: {}", e),
        }
    }
}
