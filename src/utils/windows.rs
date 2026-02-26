use windows::{
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
    },
};
use crate::utils::log::log;
use crate::utils::log::LogType::Debug;

pub fn get_active_window_info() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            let err = GetLastError();
            log(format!("GetForegroundWindow failed. Error: {:?}", err), Debug);
            return None;
        }

        log(format!("HWND: {:?}", hwnd), Debug);

        let mut pid = 0;
        let thread_id = GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if thread_id == 0 {
            let err = GetLastError();
            log(format!(
                "GetWindowThreadProcessId failed. Error: {:?}",
                err
            ), Debug);
        }

        log(format!("PID: {}", pid), Debug);


        let mut title_buf = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buf);

        let window_title = if title_len > 0 {
            String::from_utf16_lossy(&title_buf[..title_len as usize])
        } else {
            "<No title>".to_string()
        };

        Some(window_title)
    }
}

pub fn is_hd2(file_path: String) -> bool {
    let game_names: Vec<&str> = vec![
        "HELLDIVERS™ 2",
        "HELLDIVERS 2",
        "HELLDIVERS2.exe",
    ];

    if file_path == "" {
        return true;
    }

    for name in game_names {
        if file_path.to_lowercase().contains(name.to_lowercase().as_str()) {
            return true;
        }
    }

    false
}