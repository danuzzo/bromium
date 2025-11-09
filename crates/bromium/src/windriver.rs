use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use pyo3::prelude::*;
// use uiautomation::types::Handle;

use crate::sreen_context::ScreenContext;
use crate::uiauto::{get_ui_element_by_runtimeid}; // get_ui_element_by_xpath, get_element_by_xpath
use uitree::{UITreeXML, get_all_elements_xml};
// use crate::uiexplore::UITree;
use crate::app_control::launch_or_activate_application;

#[allow(unused_imports)]
use crate::commons::execute_with_timeout;
#[allow(unused_imports)]
use screen_capture::{Window, Monitor}; 

use fs_extra::dir;

use windows::Win32::Foundation::{POINT, RECT}; //HWND, 
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos}; //WindowFromPoint

use uiautomation::{UIElement}; //UIAutomation, 

use log::{debug, error, info, trace, warn};

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
        debug!("Creating new Element: name='{}', xpath='{}', handle={}", name, xpath, handle);
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
        debug!("Element::send_click called for element: {}", self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.click() {
                Ok(_) => {
                    info!("Successfully clicked on element: {:#?}", e);
                }
                Err(e) => {
                    error!("Error clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Click failed"));
                }
                
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn send_double_click(&self) -> PyResult<()> {
        debug!("Element::send_double_click called for element: {}", self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.double_click() {
                Ok(_) => {
                    info!("Double clicked on element: {:#?}", e);
                }
                Err(e) => {
                    error!("Error double clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Double click failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn send_right_click(&self) -> PyResult<()> {
        debug!("Element::send_right_click called for element: {}", self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.right_click() {
                Ok(_) => {
                    info!("Right clicked on element: {:#?}", e);
                }
                Err(e) => {
                    error!("Error right clicking on element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Right click failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn hold_click(&self, holdkeys: String) -> PyResult<()> {
        debug!("Element::hold_click called for element: {}", self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.hold_click(&holdkeys) {
                Ok(_) => {
                    info!("Hold clicked on element: {:#?}", e);
                }
                Err(e) => {
                    error!("Error hold clicking on element: {:?}", e);
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
        debug!("Element::send_keys called with keys: '{}' for element: {}", keys, self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.send_keys(&keys, 20) { // 20 ms interval for sending keys
                Ok(_) => {
                    info!("Sent keys '{}' to element: {:#?}", keys, e);
                }
                Err(e) => {
                    error!("Error sending keys to element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Send keys failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }    

    pub fn send_text(&self, text: String) -> PyResult<()> {
        debug!("Element::send_text called with text: '{}' for element: {}", text, self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.send_text(&text, 20) { // 20 ms interval for sending text
                Ok(_) => {
                    info!("Sent text '{}' to element: {:#?}", text, e);
                }
                Err(e) => {
                    error!("Error sending text to element: {:?}", e);
                    return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Send text failed"));
                }
            }
        } else {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }
        PyResult::Ok(())
    }

    pub fn hold_send_keys(&self, holdkeys: String, keys: String, interval: u64) -> PyResult<()> {
        debug!("Element::hold_send_keys called with keys: '{}' for element: {}", keys, self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.hold_send_keys(&holdkeys, &keys, interval) { // hold for the specified duration
                Ok(_) => {
                    info!("Hold sent keys '{}' to element: {:#?}", keys, e);
                }
                Err(e) => {
                    error!("Error holding send keys to element: {:?}", e);
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
        debug!("Element::show_context_menu called for element: {}", self.name);
        if let Ok(e) = convert_to_ui_element(self) {
            match e.show_context_menu() {
                Ok(_) => {
                    info!("Context menu shown for element: {:#?}", e);
                }
                Err(e) => {
                    error!("Error showing context menu for element: {:?}", e);
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
    debug!("Element::convert_to_ui_element called.");

    // First attempt: try to get the element by runtime id
    if let Some(ui_element) = get_ui_element_by_runtimeid(element.get_runtime_id()) {
        debug!("Element found by runtime id on first attempt.");
        return Ok(ui_element);
    }

    // Element not found - it may be stale. Check if auto-refresh is enabled.
    warn!("Element not found by runtime id. Element may be stale.");

    // Check if auto-refresh is enabled
    let auto_refresh_enabled = {
        let driver_guard = match WINDRIVER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("WINDRIVER lock is poisoned, recovering...");
                poisoned.into_inner()
            }
        };
        driver_guard.as_ref().map(|d| d.auto_refresh_on_stale).unwrap_or(false)
    };

    if !auto_refresh_enabled {
        error!("Element not found and auto-refresh is disabled.");
        return Err(uiautomation::Error::new(
            uiautomation::errors::ERR_NOTFOUND,
            "Element not found. Try calling driver.refresh() or enable auto-refresh with driver.set_auto_refresh(True)"
        ));
    }

    info!("Auto-refresh enabled. Attempting to refresh UI tree and retry...");

    // FIX BUG #3 & #4: Start refresh outside the lock, use timeout on recv()
    let (tx, rx): (Sender<_>, Receiver<UITreeXML>) = channel();
    thread::spawn(|| {
        debug!("Spawning thread to refresh UI tree for stale element recovery");
        get_all_elements_xml(tx, None, None);
    });

    // Wait for UI tree with timeout (10 seconds)
    let new_tree = match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(tree) => {
            info!("UI tree refreshed successfully for stale element recovery");
            tree
        }
        Err(e) => {
            error!("Failed to receive refreshed UI tree within timeout: {:?}", e);
            return Err(uiautomation::Error::new(
                uiautomation::errors::ERR_NOTFOUND,
                "Timeout waiting for UI tree refresh"
            ));
        }
    };

    // FIX BUG #5: Handle lock errors properly
    // Update the global WinDriver with the new tree
    let mut driver_guard = match WINDRIVER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            error!("WINDRIVER lock is poisoned during refresh, recovering...");
            poisoned.into_inner()
        }
    };

    if let Some(driver) = driver_guard.as_mut() {
        driver.ui_tree = new_tree;
        driver.tree_needs_update = false;
    } else {
        error!("WinDriver instance not found in global state");
        drop(driver_guard);
        return Err(uiautomation::Error::new(
            uiautomation::errors::ERR_NOTFOUND,
            "WinDriver instance not available for refresh"
        ));
    }
    drop(driver_guard);

    // FIX BUG #2: Try to recover element by XPath instead of runtime_id
    // This handles the case where UI was recreated with new runtime IDs
    info!("Attempting to find element by XPath after refresh: {}", element.get_xpath());

    let driver_guard = match WINDRIVER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };

    if let Some(driver) = driver_guard.as_ref() {
        if let Some(refreshed_elem) = driver.ui_tree.get_element_by_xpath(element.get_xpath().as_str()) {
            // Found element by XPath! Get the new runtime_id and find the UIElement
            let new_runtime_id = refreshed_elem.get_runtime_id().clone();
            drop(driver_guard);

            if let Some(ui_element) = get_ui_element_by_runtimeid(new_runtime_id) {
                info!("Element found by XPath after UI tree refresh (runtime_id may have changed).");
                return Ok(ui_element);
            }
        }
    }
    drop(driver_guard);

    // Fallback: try the old runtime_id in case it's still valid
    if let Some(ui_element) = get_ui_element_by_runtimeid(element.get_runtime_id()) {
        info!("Element found by runtime id after UI tree refresh.");
        return Ok(ui_element);
    }

    // Still not found after refresh - element truly doesn't exist
    error!("Element not found even after UI tree refresh. Element may have been removed from the UI.");
    Err(uiautomation::Error::new(
        uiautomation::errors::ERR_NOTFOUND,
        "Element not found even after automatic UI tree refresh"
    ))
}



#[pyclass]
#[derive(Debug, Clone)]
pub struct WinDriver {
    timeout_ms: u64,
    ui_tree: UITreeXML,
    tree_needs_update: bool,
    auto_refresh_on_stale: bool,
}

#[pymethods]
impl WinDriver {
    #[new]
    pub fn new(timeout_ms: u64) -> PyResult<Self> {
        debug!("Creating new WinDriver with timeout: {}ms", timeout_ms);

        // FIX BUG #6: Enforce single instance pattern
        // Check if a WinDriver instance already exists
        let existing_driver = match WINDRIVER.lock() {
            Ok(guard) => guard.is_some(),
            Err(poisoned) => {
                warn!("WINDRIVER lock is poisoned during instance check, recovering...");
                poisoned.into_inner().is_some()
            }
        };

        if existing_driver {
            error!("Attempted to create multiple WinDriver instances");
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Only one WinDriver instance can exist at a time. \
                 Please close the existing driver with driver.close() before creating a new one, \
                 or reuse the existing driver instance."
            ));
        }

        // get the ui tree in a separate thread
        let (tx, rx): (Sender<_>, Receiver<UITreeXML>) = channel();
        thread::spawn(|| {
            debug!("Spawning thread to get UI tree");
            get_all_elements_xml(tx, None, None);
        });
        info!("Spawned separate thread to get ui tree");

        // FIX BUG #4: Add timeout to recv()
        let ui_tree: UITreeXML = match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(tree) => {
                debug!("UI tree received with {} elements", tree.get_elements().len());
                tree
            }
            Err(e) => {
                error!("Timeout waiting for initial UI tree: {:?}", e);
                return Err(pyo3::exceptions::PyTimeoutError::new_err(
                    "Timeout waiting for UI tree initialization (30s)"
                ));
            }
        };

        let driver = WinDriver {
            timeout_ms,
            ui_tree,
            tree_needs_update: false,
            auto_refresh_on_stale: true, // Enable auto-refresh by default
        };

        // FIX BUG #5: Handle lock errors properly
        match WINDRIVER.lock() {
            Ok(mut guard) => {
                *guard = Some(driver.clone());
            }
            Err(poisoned) => {
                error!("WINDRIVER lock is poisoned, recovering...");
                let mut guard = poisoned.into_inner();
                *guard = Some(driver.clone());
            }
        }

        info!("WinDriver successfully created with auto-refresh enabled (singleton instance)");
        Ok(driver)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!(
            "<WinDriver timeout={}ms, auto_refresh={}, elements={}>",
            self.timeout_ms,
            self.auto_refresh_on_stale,
            self.ui_tree.get_elements().len()
        ))
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

    /// Get the current auto-refresh setting
    ///
    /// Returns:
    ///     bool: True if auto-refresh is enabled, False otherwise
    pub fn get_auto_refresh(&self) -> bool {
        self.auto_refresh_on_stale
    }

    /// Enable or disable automatic UI tree refresh when stale elements are detected
    ///
    /// When enabled (default), the library will automatically refresh the UI tree
    /// and retry operations when an element becomes stale.
    ///
    /// Args:
    ///     enabled (bool): True to enable auto-refresh, False to disable
    pub fn set_auto_refresh(&mut self, enabled: bool) {
        self.auto_refresh_on_stale = enabled;
        // FIX BUG #5: Update the global instance with proper error handling
        match WINDRIVER.lock() {
            Ok(mut guard) => {
                *guard = Some(self.clone());
            }
            Err(poisoned) => {
                error!("WINDRIVER lock is poisoned, recovering...");
                let mut guard = poisoned.into_inner();
                *guard = Some(self.clone());
            }
        }
        info!("Auto-refresh on stale elements set to: {}", enabled);
    }

    pub fn get_curser_pos(&self) -> PyResult<(i32, i32)> {
        debug!("WinDriver::get_curser_pos called.");
        let mut point = windows::Win32::Foundation::POINT { x: 0, y: 0 };
        unsafe {
            let _res= GetCursorPos(&mut point);
            PyResult::Ok((point.x, point.y))
        }
    }

    pub fn get_ui_element(&self, x: i32, y: i32) -> PyResult<Element> {
        debug!("WinDriver::get_ui_element called for coordinates: ({}, {})", x, y);

        let cursor_position = POINT { x, y };

        // FIX BUG #1: Read from global WINDRIVER to get most recent tree
        let driver_guard = match WINDRIVER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("WINDRIVER lock is poisoned, recovering...");
                poisoned.into_inner()
            }
        };

        let ui_tree = if let Some(driver) = driver_guard.as_ref() {
            &driver.ui_tree
        } else {
            warn!("No WinDriver instance in global state, using local tree");
            &self.ui_tree
        };

        if let Some(ui_element_in_tree) = crate::rectangle::get_point_bounding_rect(&cursor_position, ui_tree.get_elements()) {
            let xpath = ui_tree.get_xpath_for_element(ui_element_in_tree.get_tree_index(), true);
            trace!("Found element with xpath: {}", xpath);

            let ui_element_props = ui_element_in_tree.get_element_props();
            let ui_element_props = ui_element_props.get_element();
            let bounding_rect = ui_element_props.get_bounding_rectangle();
            let element = Element::new(
                ui_element_props.get_name().clone(),
                xpath,
                ui_element_props.get_handle(),
                ui_element_props.get_runtime_id().clone(),
                (bounding_rect.get_left(), bounding_rect.get_top(), bounding_rect.get_right(), bounding_rect.get_bottom())
            );
            info!("Successfully found element at ({}, {}): {}", x, y, element.name);
            return PyResult::Ok(element)
        } else {
            warn!("No element found at coordinates ({}, {})", x, y);
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found at the given coordinates"))
        }

    }

    fn get_ui_element_by_xpath(&self, xpath: String) -> PyResult<Element> {
        debug!("WinDriver::get_ui_element_by_xpath called.");

        // FIX BUG #1: Read from global WINDRIVER to get most recent tree
        let driver_guard = match WINDRIVER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("WINDRIVER lock is poisoned, recovering...");
                poisoned.into_inner()
            }
        };

        let ui_tree = if let Some(driver) = driver_guard.as_ref() {
            &driver.ui_tree
        } else {
            warn!("No WinDriver instance in global state, using local tree");
            &self.ui_tree
        };

        let ui_elem = ui_tree.get_element_by_xpath(xpath.as_str());
        if ui_elem.is_none() {
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Element not found"));
        }

        let element = ui_elem.unwrap();
        let name = element.get_name().clone();
        let xpath = xpath.clone();
        let handle = element.get_handle();
        let runtime_id = element.get_runtime_id().clone();
        let bounding_rectangle = element.get_bounding_rectangle();
        PyResult::Ok(Element::new(name, xpath, handle, runtime_id, (bounding_rectangle.get_left(), bounding_rectangle.get_top(), bounding_rectangle.get_right(), bounding_rectangle.get_bottom())))
    }

    pub fn get_screen_context(&self) -> PyResult<ScreenContext> {
        debug!("WinDriver::get_screen_context called.");

        let screen_context = ScreenContext::new();
        PyResult::Ok(screen_context)
    }

    pub fn take_screenshot(&self) -> PyResult<String> {
         debug!("WinDriver::take_screenshot called.");

        let monitors: Vec<Monitor>;
        if let Ok(mons) = Monitor::all() {
            if mons.is_empty() {
                error!("No monitors found for screenshot");
                return PyResult::Err(pyo3::exceptions::PyValueError::new_err("No monitors found"));
            } else {
                debug!("Found {} monitors", mons.len());
                monitors = mons;
            }
        } else {
            error!("Failed to get monitors for screenshot");
            return PyResult::Err(pyo3::exceptions::PyValueError::new_err("Failed to get monitors"));
        }

        let mut out_dir = std::env::temp_dir();
        out_dir = out_dir.join("bromium_screenshots");
        match dir::create_all(out_dir.clone(), true) {
            Ok(_) => {
                info!("Created screenshot directory at {:?}", out_dir);
            }
            Err(e) => {
                error!("Error creating screenshot directory: {:?}", e);
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
                info!("Screenshot saved successfully to: {}", filenameandpath.to_str().unwrap());
                PyResult::Ok(filenameandpath.to_str().unwrap().to_string())
            }
            Err(e) => {
                error!("Error saving screenshot: {:?}", e);
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
        debug!("WinDriver::launch_or_activate_app called with {} as app path and {} as xpath element.", app_path, xpath);

        let result = launch_or_activate_application(&app_path, &xpath);
        PyResult::Ok(result)
    }

    /// Refresh the UI tree to capture the current state of the screen
    ///
    /// This method should be called when the UI has changed significantly
    /// or when you encounter stale element errors.
    ///
    /// Returns:
    ///     None
    pub fn refresh(&mut self) -> PyResult<()> {
        debug!("WinDriver::refresh called.");
        // get the ui tree in a separate thread
        let (tx, rx): (Sender<_>, Receiver<UITreeXML>) = channel();
        thread::spawn(|| {
            debug!("Spawning thread to get UI tree");
            get_all_elements_xml(tx, None, None);
        });
        info!("Spawned separate thread to refresh ui tree");

        // FIX BUG #4: Add timeout to recv()
        let ui_tree: UITreeXML = match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(tree) => tree,
            Err(e) => {
                error!("Timeout waiting for UI tree refresh: {:?}", e);
                return Err(pyo3::exceptions::PyTimeoutError::new_err(
                    "Timeout waiting for UI tree refresh (30s)"
                ));
            }
        };

        self.ui_tree = ui_tree;
        self.tree_needs_update = false;

        // FIX BUG #5: Handle lock errors properly
        match WINDRIVER.lock() {
            Ok(mut guard) => {
                *guard = Some(self.clone());
            }
            Err(poisoned) => {
                error!("WINDRIVER lock is poisoned, recovering...");
                let mut guard = poisoned.into_inner();
                *guard = Some(self.clone());
            }
        }

        info!("UI tree refreshed successfully");
        PyResult::Ok(())
    }

    /// Close the WinDriver instance and free the global singleton
    ///
    /// This method clears the global WinDriver instance, allowing a new
    /// WinDriver to be created later if needed. After calling close(),
    /// this driver instance and any elements created from it should not
    /// be used.
    ///
    /// Returns:
    ///     None
    pub fn close(&mut self) -> PyResult<()> {
        debug!("WinDriver::close called.");

        // Clear the global WINDRIVER instance
        match WINDRIVER.lock() {
            Ok(mut guard) => {
                *guard = None;
                info!("WinDriver instance closed and global singleton cleared");
            }
            Err(poisoned) => {
                warn!("WINDRIVER lock is poisoned during close, recovering...");
                let mut guard = poisoned.into_inner();
                *guard = None;
                info!("WinDriver instance closed and global singleton cleared (after lock recovery)");
            }
        }

        PyResult::Ok(())
    }
}

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}