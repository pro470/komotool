use std::ffi::OsString;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
        System::Threading::*,
    },
};
use bevy::prelude::*;
use thiserror::Error;

// Custom error type
#[derive(Debug, Error)]
pub enum WindowError {
    #[error("Windows API error: {0}")]
    WinApi(#[from] windows::core::Error),
    #[error("Process access denied")]
    AccessDenied,
    #[error("Invalid window handle")]
    InvalidHandle,
}

#[derive(Debug, Clone, Component)]
pub struct WindowInfo {
    pub hwnd: isize,
    pub title: String,
    pub class: String,
    pub pid: u32,
    pub exe_path: String,
    pub is_visible: bool,
}

pub struct SystemTheme {
    pub dark_mode: bool,
    pub accent_color: [u8; 4],
    pub transparency: bool,
}

#[derive(Resource, Default)]
pub struct WindowList(pub Vec<WindowInfo>);

// Core window enumeration functionality
pub fn list_windows() -> Result<Vec<WindowInfo>> {
    unsafe {
        let mut windows = Vec::new();
        let mut data = EnumWindowsData { windows: &mut windows };

        EnumWindows(
            Some(enum_callback),
            LPARAM(&mut data as *mut _ as _)
        )?;

        Ok(windows)
    }
}

struct EnumWindowsData<'a> {
    windows: &'a mut Vec<WindowInfo>,
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = &mut *(lparam.0 as *mut EnumWindowsData<'_>);

    // Skip invisible windows
    if IsWindowVisible(hwnd).as_bool() {
        match get_window_info(hwnd) {
            Ok(info) => data.windows.push(info),
            Err(e) => error!("Error getting window info: {}", e),
        }
    }

    // Continue enumeration
    BOOL::from(true)
}

fn get_window_info(hwnd: HWND) -> Result<WindowInfo> {
    unsafe {
        // Get window title
        let mut title_buffer = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buffer);
        let title = String::from_utf16_lossy(&title_buffer[..title_len as usize]);

        // Get window class
        let mut class_buffer = [0u16; 256];
        let class_len = GetClassNameW(hwnd, &mut class_buffer);
        let class = String::from_utf16_lossy(&class_buffer[..class_len as usize]);

        // Get process ID
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        // Get executable path
        let exe_path = get_process_path(pid)?;

        // Check window visibility
        let is_visible = IsWindowVisible(hwnd).as_bool();

        Ok(WindowInfo {
            hwnd: hwnd.0,
            title,
            class,
            pid,
            exe_path,
            is_visible,
        })
    }
}

fn get_process_path(pid: u32) -> Result<String> {
    unsafe {
        let process = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            pid
        ).map_err(|_| WindowError::AccessDenied)?;

        let mut buffer = [0u16; MAX_PATH as usize];
        let mut size = buffer.len() as u32;

        QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_NATIVE,
            &mut buffer,
            &mut size
        )?;

        CloseHandle(process)?;

        Ok(OsString::from_wide(&buffer[..size as usize])
            .to_string_lossy()
            .into_owned())
    }
}

// Bevy system implementation
pub fn update_window_list(mut windows: ResMut<WindowList>) {
    match list_windows() {
        Ok(new_windows) => {
            // Simple change detection
            if new_windows != windows.0 {
                windows.0 = new_windows;
            }
        }
        Err(e) => error!("Failed to update window list: {}", e),
    }
}