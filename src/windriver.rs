use crate::context::ScreenContext;


use pyo3::prelude::*;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, WindowFromPoint};

use uiautomation::types::Handle;
use uiautomation::{UIAutomation, UIElement};



#[pyclass]
#[derive(Debug)]
pub struct Element {
    name: String,
    // hwnd: HWND,
}

#[pymethods]
impl Element {
    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<Element name={}>", self.name))
    }

    pub fn __str__(&self) -> PyResult<String> {
        PyResult::Ok(self.name.clone())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
}

#[pyclass]
#[derive(Debug)]
pub struct WinDriver {
    timeout: usize,
}

#[pymethods]
impl WinDriver {
    #[new]
    pub fn new(timeout: usize) -> PyResult<Self> {
        Ok(WinDriver { timeout })
    }

    fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<WinDriver timeout={}>", self.timeout))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    pub fn get_timeout(&self) -> usize {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: usize) {
        self.timeout = timeout;
    }

    pub fn get_curser_pos(&self) -> PyResult<(i32, i32)> {
        let mut point = windows::Win32::Foundation::POINT { x: 0, y: 0 };
        unsafe {
            let _res= GetCursorPos(&mut point);
            PyResult::Ok((point.x, point.y))
        }
    }

    pub fn get_ui_element(&self, x: i32, y: i32) -> PyResult<Element> {

        
        // Initialize UIAutomation
        let uia = UIAutomation::new().unwrap();

        let name: String;
        let point = windows::Win32::Foundation::POINT { x, y };
        let hwnd: HWND;

        unsafe {
            // let _res= GetCursorPos(&mut point);
            hwnd = WindowFromPoint(point);

            let handle: Handle = Handle::from(hwnd.0 as isize);
            let element: Result<UIElement, uiautomation::Error> = uia.element_from_handle(handle);
            match element {
                Ok(e) => {
                    name = e.get_name().unwrap_or("".to_string());
                }
                Err(_e) => {
                    name = "invalid hwnd".to_string();
                }
            }
        }

        PyResult::Ok(Element {
            name: name,
        })
    }

    pub fn get_screen_context(&self) -> PyResult<ScreenContext> {
        let screen_context = ScreenContext::new();
        PyResult::Ok(screen_context)
    }
}