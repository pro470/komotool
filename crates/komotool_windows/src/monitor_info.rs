use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use bevy_ecs::component::Component;
use bevy_ecs::system::Resource;
use thiserror::Error;
use tracing::error;
use windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI},
    },
};

#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("Windows API error: {0}")]
    WinApi(#[from] windows::core::Error),
    #[error("Invalid monitor handle")]
    InvalidHandle,
}

#[derive(Debug, Clone, Component)]
pub struct MonitorInfo {
    pub handle: isize,
    pub device_name: String,
    pub width: i32,
    pub height: i32,
    pub work_area: (i32, i32, i32, i32), // (left, top, right, bottom)
    pub is_primary: bool,
    pub dpi_x: u32,
    pub dpi_y: u32,
}

#[derive(Resource, Default)]
pub struct MonitorList(pub Vec<MonitorInfo>);

pub fn list_monitors() -> std::result::Result<Vec<MonitorInfo>, MonitorError> {
    unsafe {
        let mut monitors = Vec::new();
        let mut data = EnumMonitorsData {
            monitors: &mut monitors,
        };

        EnumDisplayMonitors(
            Some(HDC::default()),
            None,
            Some(monitor_enum_callback),
            LPARAM(&mut data as *mut _ as _),
        )
        .ok()
        .map_err(|_| MonitorError::WinApi(windows::core::Error::from_win32()))?;

        Ok(monitors)
    }
}

struct EnumMonitorsData<'a> {
    monitors: &'a mut Vec<MonitorInfo>,
}

unsafe extern "system" fn monitor_enum_callback(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let data = &mut *(lparam.0 as *mut EnumMonitorsData<'_>);

    let mut info: MONITORINFOEXW = MONITORINFOEXW {
        monitorInfo: MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFOEXW>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        },
        szDevice: [0; 32],
    };

    if GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut MONITORINFO).as_bool() {
        // Add DPI detection
        let mut dpi_x = 96;
        let mut dpi_y = 96;

        if let Err(e) = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) {
            error!("Failed to get DPI for monitor: {}", e);
        }

        let device_name = OsString::from_wide(&info.szDevice)
            .to_string_lossy()
            .into_owned();

        data.monitors.push(MonitorInfo {
            handle: hmonitor.0 as isize,
            device_name,
            width: info.monitorInfo.rcMonitor.right - info.monitorInfo.rcMonitor.left,
            height: info.monitorInfo.rcMonitor.bottom - info.monitorInfo.rcMonitor.top,
            work_area: (
                info.monitorInfo.rcWork.left,
                info.monitorInfo.rcWork.top,
                info.monitorInfo.rcWork.right,
                info.monitorInfo.rcWork.bottom,
            ),
            is_primary: (info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0,
            dpi_x,
            dpi_y,
        });
    }

    TRUE
}
