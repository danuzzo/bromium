use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows::Win32::Graphics::Gdi::{MONITOR_FROM_FLAGS, MonitorFromPoint};
use windows::Win32::UI::HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE, MONITOR_DPI_TYPE, GetDpiForMonitor, SetProcessDpiAwarenessContext, GetDpiAwarenessContextForProcess, GetAwarenessFromDpiAwarenessContext}; //DPI_AWARENESS, DPI_AWARENESS_CONTEXT, GetThreadDpiAwarenessContext
use windows::Win32::Foundation::{POINT, HANDLE};

use pyo3::prelude::*;


#[repr(C)]
struct ScreenSize {
    width: i32,
    height: i32,
}

#[pyclass]
#[derive(Debug)]
// #[repr(C)]
pub struct ScreenContext {
    screen_width: i32,
    screen_height: i32,
    screen_scale: f32,
}

#[pymethods]
impl ScreenContext {
    #[new]
    pub fn new() -> Self {

        let screen_size = get_system_metrics();
        let screen_width = screen_size.width;
        let screen_height = screen_size.height; 
        let screen_scale = get_screen_scale_factor();

        Self {
            screen_width,
            screen_height,
            screen_scale,
        }
    }    

    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<ScreenContext screen_width={} screen_height={} screen_scale={}>", self.screen_width, self.screen_height, self.screen_scale))
    }

    pub fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    pub fn get_screen_width(&self) -> i32 {
        self.screen_width
    }

    pub fn get_screen_height(&self) -> i32 {
        self.screen_height
    }

    pub fn get_screen_scale(&self) -> f32 {
        self.screen_scale
    }

}

fn get_system_metrics() -> ScreenSize {
    unsafe {
        let x = GetSystemMetrics(SM_CXSCREEN);
        let y = GetSystemMetrics(SM_CYSCREEN);
        // println!("Screen size: {}x{}", x, y);
        ScreenSize { width: x, height: y }
    }
}

fn get_screen_scale_factor() -> f32 {

    unsafe {
        // First we need to set the DPI awareness context to per monitor aware
        // This is required to get the correct DPI for the monitor
        let monitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_FROM_FLAGS { 0: 2 });
        let _res_dpi_awareness_context = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE);
        let dpi_awareness_process = GetDpiAwarenessContextForProcess(HANDLE(std::ptr::null_mut()));
        let _awareness_process = GetAwarenessFromDpiAwarenessContext(dpi_awareness_process);

        let mut dpi_x = 0;
        let mut dpi_y = 0;
        let _res = GetDpiForMonitor(monitor, MONITOR_DPI_TYPE {0: 0}, &mut dpi_x, &mut dpi_y);

        let scale_x = dpi_x as f32 / 96.0;
        let scale_y = dpi_y as f32 / 96.0;
        let scale = (scale_x + scale_y) / 2.0;

        scale
    }


}