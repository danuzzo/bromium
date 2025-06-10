use crate::context::ScreenContext;
use crate::uiauto::{get_ui_element_by_runtimeid, get_ui_element_by_xpath, get_element_by_xpath};
use crate::xpath::generate_xpath;
use crate::app_control::launch_or_activate_application;
use crate::logging::PerformanceTimer;

#[allow(unused_imports)]
use crate::commons::execute_with_timeout;

use pyo3::prelude::*;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, WindowFromPoint};

use uiautomation::types::Handle;
use uiautomation::{UIAutomation, UIElement};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Element {
    name: String,
    xpath: String,
    handle: isize,
    runtime_id: Vec<i32>,
    bounding_rectangle: RECT,
}

#[pymethods]
impl Element {

    #[new]
    pub fn new(name: String, xpath: String, handle: isize, runtime_id: Vec<i32>, bounding_rectangle: (i32, i32, i32, i32)) -> Self {
        let bounding_rectangle  = RECT {
            left: bounding_rectangle.0,
            top: bounding_rectangle.1,
            right: bounding_rectangle.2,
            bottom: bounding_rectangle.3,
        };
        
        log::debug!("Created new Element: name='{}', handle={}, runtime_id={:?}, bounds=({},{},{},{})", 
                   name, handle, runtime_id, 
                   bounding_rectangle.left, bounding_rectangle.top, 
                   bounding_rectangle.right, bounding_rectangle.bottom);
        
        Element { name, xpath, handle, runtime_id , bounding_rectangle}
    }

    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<Element\nname='{}'\nhandle = {}\nruntime_id = {:?}\nbounding_rectangle = {:?}>", self.name, self.handle, self.runtime_id, self.bounding_rectangle))
    }

    pub fn __str__(&self) -> PyResult<String> {
        PyResult::Ok(self.name.clone())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_xpath(&self) -> String {
        self.xpath.clone()
    }

    pub fn get_handle(&self) -> isize {
        self.handle
    }

    pub fn get_runtime_id(&self) -> Vec<i32> {
        self.runtime_id.clone()
    }
    
    pub fn send_click(&self) -> PyResult<()> {
        let _timer = PerformanceTimer::new("element_send_click");
        log::info!("Attempting to click element: name='{}', runtime_id={:?}", self.name, self.runtime_id);
        
        if let Ok(e) = convert_to_ui_element(self) {
            log::debug!("Successfully converted to UIElement, performing click");
            match e.click() {
                Ok(_) => {
                    log::info!("Successfully clicked element: name='{}'", self.name);
                }
                Err(e) => {
                    log::error!("Failed to click element '{}': {}", self.name, e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Click failed"));
                }
            }
        } else {
            log::error!("Failed to convert Element to UIElement for clicking: name='{}'", self.name);
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }
}

impl Default for Element {
    fn default() -> Self {
        Element {
            name: String::new(),
            xpath: String::new(),
            handle: 0,
            runtime_id: vec![],
            bounding_rectangle: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
        }
    }
}

fn convert_to_ui_element(element: &Element) -> Result<UIElement, uiautomation::Error> {
    let _timer = PerformanceTimer::new("convert_to_ui_element");
    log::debug!("Converting Element to UIElement: name='{}', runtime_id={:?}", element.name, element.runtime_id);

    // first try to get the element by runtime id
    if let Some(ui_element) = get_ui_element_by_runtimeid(element.get_runtime_id()) {
        log::debug!("Successfully found element by runtime ID");
        return Ok(ui_element);
    } else {
        log::debug!("Runtime ID lookup failed, trying XPath lookup");
        // if that fails, try to get the element by xpath
        if let Some(ui_element) = get_ui_element_by_xpath(element.get_xpath()) {
            log::debug!("Successfully found element by XPath");
            return Ok(ui_element);
        } else {
            log::error!("Failed to find element by both runtime ID and XPath");
            return Err(uiautomation::Error::new(uiautomation::errors::ERR_NOTFOUND, "could not find element"));
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct WinDriver {
    timeout_ms: u64,
}

#[pymethods]
impl WinDriver {
    #[new]
    pub fn new(timeout_ms: u64) -> PyResult<Self> {
        log::info!("Creating new WinDriver with timeout: {}ms", timeout_ms);
        Ok(WinDriver { timeout_ms })
    }

    fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<WinDriver timeout={}>", self.timeout_ms))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout_ms
    }

    pub fn set_timeout(&mut self, timeout_ms: u64) {
        log::info!("Updating WinDriver timeout from {}ms to {}ms", self.timeout_ms, timeout_ms);
        self.timeout_ms = timeout_ms;
    }

    pub fn get_curser_pos(&self) -> PyResult<(i32, i32)> {
        let _timer = PerformanceTimer::new("get_cursor_pos");
        log::debug!("Getting current cursor position");
        
        let mut point = windows::Win32::Foundation::POINT { x: 0, y: 0 };
        unsafe {
            let res = GetCursorPos(&mut point);
            if res.is_ok() {
                log::debug!("Cursor position: ({}, {})", point.x, point.y);
                PyResult::Ok((point.x, point.y))
            } else {
                log::error!("Failed to get cursor position");
                PyResult::Err(pyo3::exceptions::PyRuntimeError::new_err("Failed to get cursor position"))
            }
        }
    }

    pub fn get_ui_element(&self, x: i32, y: i32) -> PyResult<Element> {
        let _timer = PerformanceTimer::new("get_ui_element");
        log::info!("Getting UI element at coordinates: ({}, {})", x, y);

        // Initialize UIAutomation
        let uia = UIAutomation::new().unwrap();

        let name: String;
        let xpath: String;
        let point = windows::Win32::Foundation::POINT { x, y };
        let hwnd: HWND;
        let handle: isize;
        let runtime_id: Vec<i32>;  
        let bounding_rectangle: uiautomation::types::Rect;

        unsafe {
            hwnd = WindowFromPoint(point);
            log::debug!("Window handle from point ({}, {}): {:?}", x, y, hwnd);
            
            handle = hwnd.0 as isize;
            let elem_handle: Handle = Handle::from(hwnd.0 as isize);
            
            log::debug!("Attempting to get UIElement from handle: {}", handle);
            let element: Result<UIElement, uiautomation::Error> = uia.element_from_handle(elem_handle);
            
            match element {
                Ok(e) => {
                    name = e.get_name().unwrap_or("".to_string());
                    log::debug!("Found element with name: '{}'", name);
                    
                    log::debug!("Generating XPath for coordinates ({}, {})", x, y);
                    xpath = generate_xpath(x, y);
                    
                    runtime_id = e.get_runtime_id().unwrap_or_default();
                    log::debug!("Element runtime ID: {:?}", runtime_id);
                    
                    bounding_rectangle = e.get_bounding_rectangle().unwrap_or_default();
                    log::debug!("Element bounds: ({}, {}, {}, {})", 
                               bounding_rectangle.get_left(), bounding_rectangle.get_top(),
                               bounding_rectangle.get_right(), bounding_rectangle.get_bottom());
                }
                Err(e) => {
                    log::error!("Failed to get UIElement from handle {}: {}", handle, e);
                    name = "invalid hwnd".to_string();
                    xpath = "no xpath found".to_string();
                    runtime_id = vec![];
                    bounding_rectangle = uiautomation::types::Rect::default();
                }
            }
        }

        let element = Element {
            name: name.clone(),
            xpath: xpath.clone(),
            handle: handle,
            runtime_id: runtime_id.clone(),
            bounding_rectangle: RECT { 
                left: bounding_rectangle.get_left(), 
                top: bounding_rectangle.get_top(), 
                right: bounding_rectangle.get_right(), 
                bottom: bounding_rectangle.get_bottom() 
            },
        };
        
        log::info!("Successfully created element: name='{}', handle={}, runtime_id={:?}", 
                  name, handle, runtime_id);
        
        PyResult::Ok(element)
    }

    fn get_ui_element_by_xpath(&self, xpath: String) -> PyResult<Element> {
        let _timer = PerformanceTimer::new("get_ui_element_by_xpath");
        log::info!("Getting UI element by XPath (length: {})", xpath.len());
        log::debug!("XPath: {}", if xpath.len() > 200 { &xpath[..200] } else { &xpath });
        
        let ui_elem = get_element_by_xpath(xpath.clone());
        if ui_elem.is_none() {
            log::error!("Failed to find element by XPath");
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        let element = ui_elem.unwrap();
        
        log::info!("Successfully found element by XPath: name='{}'", element.get_name());
        PyResult::Ok(element)
    }

    pub fn get_screen_context(&self) -> PyResult<ScreenContext> {
        let _timer = PerformanceTimer::new("get_screen_context");
        log::debug!("Getting screen context information");
        
        let screen_context = ScreenContext::new();
        
        log::debug!("Screen context: width={}, height={}, scale={}", 
                   screen_context.get_screen_width(), 
                   screen_context.get_screen_height(), 
                   screen_context.get_screen_scale());
        
        PyResult::Ok(screen_context)
    }

    /// Launch or activate an application using its path and an XPath
    /// 
    /// Args:
    ///     app_path (str): Full path to the application executable
    ///     xpath (str): XPath that identifies an element in the application window
    /// 
    /// Returns:
    ///     bool: True if the application was successfully launched or activated
    pub fn launch_or_activate_app(&self, app_path: String, xpath: String) -> PyResult<bool> {
        let _timer = PerformanceTimer::new("launch_or_activate_app");
        log::info!("Launching or activating application: path='{}', xpath_length={}", 
                  app_path, xpath.len());
        log::debug!("Application XPath: {}", if xpath.len() > 200 { &xpath[..200] } else { &xpath });
        
        let result = launch_or_activate_application(&app_path, &xpath);
        
        if result {
            log::info!("Successfully launched or activated application: {}", app_path);
        } else {
            log::error!("Failed to launch or activate application: {}", app_path);
        }
        
        PyResult::Ok(result)
    }
}