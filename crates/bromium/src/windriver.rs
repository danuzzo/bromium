use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};

use pyo3::prelude::*;

use crate::context::ScreenContext;
use crate::printfmt;
use crate::uiauto::{get_ui_element_by_runtimeid, get_ui_element_by_xpath, get_element_by_xpath};
use crate::uiexplore::UITree;
use crate::app_control::launch_or_activate_application;

#[allow(unused_imports)]
use crate::commons::execute_with_timeout;
#[allow(unused_imports)]
use screen_capture::{Window, Monitor}; 

use fs_extra::dir;

use windows::Win32::Foundation::{RECT, POINT}; //HWND, 
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos}; //WindowFromPoint

use uiautomation::{UIElement}; //UIAutomation, 

static WINDRIVER: Mutex<Option<WinDriver>> = Mutex::new(None);


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
    
    // Region mouse methods
    pub fn send_click(&self) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.click() {
                Ok(_) => {
                    printfmt!("Clicked on element: {:#?}", e);
                }
                Err(e) => {
                    printfmt!("Error clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Click failed"));
                }
                
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn send_double_click(&self) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.double_click() {
                Ok(_) => {
                    printfmt!("Double clicked on element: {:#?}", e);
                }
                Err(e) => {
                    printfmt!("Error double clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Double click failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn send_right_click(&self) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.right_click() {
                Ok(_) => {
                    printfmt!("Right clicked on element: {:#?}", e);
                }
                Err(e) => {
                    printfmt!("Error right clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Right click failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn hold_click(&self, holdkeys: String) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.hold_click(&holdkeys) {
                Ok(_) => {
                    printfmt!("Hold clicked on element: {:#?}", e);
                }
                Err(e) => {
                    printfmt!("Error hold clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Hold click failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    // Region keyboard methods
    pub fn send_keys(&self, keys: String) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.send_keys(&keys, 20) { // 20 ms interval for sending keys
                Ok(_) => {
                    printfmt!("Sent keys '{}' to element: {:#?}", keys, e);
                }
                Err(e) => {
                    printfmt!("Error sending keys to element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Send keys failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }    

    pub fn send_text(&self, text: String) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.send_text(&text, 20) { // 20 ms interval for sending text
                Ok(_) => {
                    printfmt!("Sent text '{}' to element: {:#?}", text, e);
                }
                Err(e) => {
                    printfmt!("Error sending text to element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Send text failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn hold_send_keys(&self, holdkeys: String, keys: String, interval: u64) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.hold_send_keys(&holdkeys, &keys, interval) { // hold for the specified duration
                Ok(_) => {
                    printfmt!("Hold sent keys '{}' to element: {:#?}", keys, e);
                }
                Err(e) => {
                    printfmt!("Error holding send keys to element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Hold send keys failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    // Region misc methods
    pub fn show_context_menu(&self) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.show_context_menu() {
                Ok(_) => {
                    printfmt!("Context menu shown for element: {:#?}", e);
                }
                Err(e) => {
                    printfmt!("Error showing context menu for element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Show context menu failed"));
                }
            }
        } else {
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

    // first try to get the element by runtime id
    if let Some(ui_element) = get_ui_element_by_runtimeid(element.get_runtime_id()) {
        return Ok(ui_element);
    } else {
        // if that fails, try to get the element by xpath

        // get the WINDRIVER context and get the xpath from the element and the context
        {
            let guard = WINDRIVER.lock().unwrap();
            let windriver = guard.as_ref().ok_or_else(|| uiautomation::Error::new(uiautomation::errors::ERR_NOTFOUND, "WinDriver not initialized"))?;
            let ui_tree = &windriver.ui_tree;
            if let Some(ui_element) = get_ui_element_by_xpath(element.get_xpath(), ui_tree) {
                return Ok(ui_element);
            } else {
                return Err(uiautomation::Error::new(uiautomation::errors::ERR_NOTFOUND, "could not find element"));
            }
        }
    }

}



#[pyclass]
#[derive(Debug, Clone)]
pub struct WinDriver {
    timeout_ms: u64,
    ui_tree: UITree,
    tree_needs_update: bool,
    // TODO: Add screen context to get scaling factor later on
}

#[pymethods]
impl WinDriver {
    #[new]
    pub fn new(timeout_ms: u64) -> PyResult<Self> {
        
        // get the ui tree in a separate thread
        let (tx, rx): (Sender<_>, Receiver<crate::UITree>) = channel();
        thread::spawn(|| {
            crate::get_all_elements(tx, None);
        });
        printfmt!("Spawned separate thread to get ui tree");
        
        let ui_tree: UITree = rx.recv().unwrap();
        let driver = WinDriver { timeout_ms, ui_tree, tree_needs_update: false };

        *WINDRIVER.lock().unwrap() = Some(driver.clone());

        Ok(driver)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<WinDriver timeout={}>, ui_tree={{object}}, needs_update={}", self.timeout_ms, self.tree_needs_update))
    }

    pub fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout_ms
    }

    pub fn set_timeout(&mut self, timeout_ms: u64) {
        self.timeout_ms = timeout_ms;
    }

    pub fn get_curser_pos(&self) -> PyResult<(i32, i32)> {
        let mut point = windows::Win32::Foundation::POINT { x: 0, y: 0 };
        unsafe {
            let _res= GetCursorPos(&mut point);
            PyResult::Ok((point.x, point.y))
        }
    }

    pub fn get_ui_element(&self, x: i32, y: i32) -> PyResult<Element> {
    
        let cursor_position = POINT { x, y };

        if let Some(ui_element_in_tree) = crate::rectangle::get_point_bounding_rect(&cursor_position, self.ui_tree.get_elements()) {
            let xpath = self.ui_tree.get_xpath_for_element(ui_element_in_tree.get_tree_index());
            let ui_element_props = ui_element_in_tree.get_element_props();
            let element = Element::new(
                ui_element_props.name.clone(),
                xpath,
                ui_element_props.handle,
                ui_element_props.runtime_id.clone(),
                (ui_element_props.bounding_rect.get_left(), ui_element_props.bounding_rect.get_top(), ui_element_props.bounding_rect.get_right(), ui_element_props.bounding_rect.get_bottom())
            );
            return PyResult::Ok(element)
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found at the given coordinates"))
        }

    }

    fn get_ui_element_by_xpath(&self, xpath: String) -> PyResult<Element> {
        
        let ui_elem = get_element_by_xpath(xpath.clone(), &self.ui_tree);
        if ui_elem.is_none() {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        let element = ui_elem.unwrap();
        PyResult::Ok(element)
        // let name = element.get_name();
        // let xpath = element.get_xpath();
        // let handle = element.get_handle();
        // PyResult::Ok(Element { name, xpath, handle})
    }

    pub fn get_screen_context(&self) -> PyResult<ScreenContext> {
        let screen_context = ScreenContext::new();
        PyResult::Ok(screen_context)
    }

    pub fn take_screenshot(&self) -> PyResult<String> {
 
        let monitors: Vec<Monitor>;
        if let Ok(mons) = Monitor::all() {
            if mons.is_empty() {
                return PyResult::Err(pyo3::exceptions::PyValueError::new_err("No monitors found"));
            } else {
                monitors = mons;
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Failed to get monitors"));
        }

        let mut out_dir = std::env::temp_dir();
        out_dir = out_dir.join("bromium_screenshots");
        match dir::create_all(out_dir.clone(), true) {
            Ok(_) => {
                printfmt!("Created screenshot directory at {:?}", out_dir);
            }
            Err(e) => {
                printfmt!("Error creating screenshot directory: {:?}", e);
                return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Failed to create screenshot directory"));
            }
        }
        
        let primary_monitor: Option<Monitor> = monitors.into_iter().find(|m| m.is_primary().unwrap_or(false));
        if primary_monitor.is_none() {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("No primary monitor found"));
        }
        
        let monitor = primary_monitor.unwrap();
        let image = monitor.capture_image().unwrap();
        let filename = format!(
            "monitor-{}.png",
            normalized(monitor.name().unwrap()));
        let filenameandpath = out_dir.join(filename);
        match image.save(filenameandpath.clone()) {
            Ok(_) => {
                printfmt!("Screenshot saved successfully");
                PyResult::Ok(filenameandpath.to_str().unwrap().to_string())
            }
            Err(e) => {
                printfmt!("Error saving screenshot: {:?}", e);
                PyResult::Err(pyo3::exceptions::PyValueError::new_err("Failed to save screenshot"))
            }
        }
        
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
        let result = launch_or_activate_application(&app_path, &xpath);
        PyResult::Ok(result)
    }

    fn refresh(&mut self) -> PyResult<()> {
        // get the ui tree in a separate thread
        let (tx, rx): (Sender<_>, Receiver<crate::UITree>) = channel();
        thread::spawn(|| {
            crate::get_all_elements(tx, None);
        });
        printfmt!("Spawned separate thread to refresh ui tree");
        
        let ui_tree: UITree = rx.recv().unwrap();
        
        self.ui_tree = ui_tree;
        self.tree_needs_update = false;
        
        {
            *WINDRIVER.lock().unwrap() = Some(self.clone());
        }

        PyResult::Ok(())
    }
}

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}