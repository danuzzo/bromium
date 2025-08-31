// use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
// use windows::Win32::Graphics::Gdi::{MONITOR_FROM_FLAGS, MonitorFromPoint};
// use windows::Win32::UI::HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE, MONITOR_DPI_TYPE, GetDpiForMonitor, SetProcessDpiAwarenessContext, GetDpiAwarenessContextForProcess, GetAwarenessFromDpiAwarenessContext}; //DPI_AWARENESS, DPI_AWARENESS_CONTEXT, GetThreadDpiAwarenessContext
// use windows::Win32::Foundation::{POINT, HANDLE};

use display_info::DisplayInfo;

use pyo3::prelude::*;


// #[repr(C)]
// struct ScreenSize {
//     width: i32,
//     height: i32,
// }
#[pyclass]
#[derive(Debug, Clone)]
pub struct ScreenInfo {
    /// Unique identifier associated with the display.
    pub id: u32,
    /// The display name
    pub name: String,
    /// The display friendly name
    pub friendly_name: String,
    /// The display x coordinate.
    pub x: i32,
    /// The display x coordinate.
    pub y: i32,
    /// The display pixel width.
    pub width: u32,
    /// The display pixel height.
    pub height: u32,
    /// The width of a display in millimeters. This value may be 0.
    pub width_mm: i32,
    /// The height of a display in millimeters. This value may be 0.
    pub height_mm: i32,
    /// Can be 0, 90, 180, 270, represents screen rotation in clock-wise degrees.
    pub rotation: f32,
    /// Output device's pixel scale factor.
    pub scale_factor: f32,
    /// The display refresh rate.
    pub frequency: f32,
    /// Whether the screen is the main screen
    pub is_primary: bool,
}

impl From<DisplayInfo> for ScreenInfo {
    fn from(info: DisplayInfo) -> Self {
        Self {
            id: info.id,
            name: info.name,
            friendly_name: info.friendly_name,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            width_mm: info.width_mm,
            height_mm: info.height_mm,
            rotation: info.rotation,
            scale_factor: info.scale_factor,
            frequency: info.frequency,
            is_primary: info.is_primary,
        }
    }
}

#[pymethods]
impl ScreenInfo {
    #[new]
    pub fn new(
        id: u32,
        name: String,
        friendly_name: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        width_mm: i32,
        height_mm: i32,
        rotation: f32,
        scale_factor: f32,
        frequency: f32,
        is_primary: bool,
    ) -> Self {
        Self {
            id,
            name,
            friendly_name,
            x,
            y,
            width,
            height,
            width_mm,
            height_mm,
            rotation,
            scale_factor,
            frequency,
            is_primary,
        }
    }
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_friendly_name(&self) -> &str {
        &self.friendly_name
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_width_mm(&self) -> i32 {
        self.width_mm
    }
    pub fn get_height_mm(&self) -> i32 {
        self.height_mm
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn get_scale_factor(&self) -> f32 {
        self.scale_factor
    }
    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }
    pub fn __repr__(&self) -> String {
        format!(
            "<ScreenInfo id={} name={} friendly_name={} x={} y={} width={} height={} width_mm={} height_mm={} rotation={} scale_factor={} frequency={} is_primary={}>",
            self.id, self.name, self.friendly_name, self.x, self.y, self.width, self.height, self.width_mm, self.height_mm, self.rotation, self.scale_factor, self.frequency, self.is_primary
        )
    }
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct ScreenContext {
    screens: Vec<ScreenInfo>,
    primary_screen: ScreenInfo,
}

#[pymethods]
impl ScreenContext {
    #[new]
    pub fn new() -> Self {

        let displays = DisplayInfo::all().unwrap_or(Vec::new());
        
    let screens: Vec<ScreenInfo> = displays.into_iter()
        .map(|display| ScreenInfo::from(display))
        .collect();


        let primary_screen = screens.iter()
        .find(|screen| screen.is_primary)
        .cloned()
        .unwrap_or_else(|| screens.first().cloned().expect("No screens found"));

        Self {
            screens,
            primary_screen,
            // screen_width,
            // screen_height,
            // screen_scale,
        }
    } 

    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<ScreenContext primary_screen={} screens_count={}>", self.primary_screen.name, self.screens.len()))
    }
    pub fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
    pub fn get_primary_screen(&self) -> PyResult<ScreenInfo> {
        Ok(self.primary_screen.clone())
    }
    pub fn get_screens(&self) -> PyResult<Vec<ScreenInfo>> {
        Ok(self.screens.clone())
    }

}


/*
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
*/