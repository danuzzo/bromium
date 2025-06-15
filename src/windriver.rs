use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};

use pyo3::prelude::*;

use crate::context::ScreenContext;
use crate::uiauto::{get_ui_element_by_runtimeid, get_ui_element_by_xpath, get_element_by_xpath};
use crate::uiexplore::UITree;
use crate::app_control::launch_or_activate_application;

#[allow(unused_imports)]
use crate::commons::execute_with_timeout;


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
    
    pub fn send_click(&self) -> PyResult<()> {
        if let Ok(e) = convert_to_ui_element(self) {
            match e.click() {
                Ok(_) => {
                    println!("Clicked on element: {:#?}", e);
                }
                Err(e) => {
                    println!("Error clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Click failed"));
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
    needs_update: bool,
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
        println!("Spawned separate thread to get ui tree");
        
        let ui_tree: UITree = rx.recv().unwrap();
        let driver = WinDriver { timeout_ms, ui_tree, needs_update: false };

        *WINDRIVER.lock().unwrap() = Some(driver.clone());

        Ok(driver)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("<WinDriver timeout={}>, ui_tree={{object}}, needs_update={}", self.timeout_ms, self.needs_update))
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
        println!("Spawned separate thread to refresh ui tree");
        
        let ui_tree: UITree = rx.recv().unwrap();
        
        self.ui_tree = ui_tree;
        self.needs_update = false;
        
        {
            *WINDRIVER.lock().unwrap() = Some(self.clone());
        }

        PyResult::Ok(())
    }
}

