use crate::windriver::Element;
use crate::xpath::generate_xpath;
use crate::xpath::get_path_to_element;
use crate::xpath::XpathElement;
use crate::logging::PerformanceTimer;
use crate::{log_uiauto_operation};
use uiautomation::{controls::ControlType, UIAutomation, UIElement};

trait ConvertToControlType {
    fn from_str(item: &str) -> ControlType;
}

impl ConvertToControlType for ControlType {
    fn from_str(item: &str) -> Self {
        let control_type = match item {
            "Button"  => ControlType::Button,
            "Calendar"  => ControlType::Calendar,
            "CheckBox"  => ControlType::CheckBox,
            "ComboBox"  => ControlType::ComboBox,
            "Edit"  => ControlType::Edit,
            "Hyperlink"  => ControlType::Hyperlink,
            "Image"  => ControlType::Image,
            "ListItem"  => ControlType::ListItem,
            "List"  => ControlType::List,
            "Menu"  => ControlType::Menu,
            "MenuBar"  => ControlType::MenuBar,
            "MenuItem"  => ControlType::MenuItem,
            "ProgressBar"  => ControlType::ProgressBar,
            "RadioButton"  => ControlType::RadioButton,
            "ScrollBar"  => ControlType::ScrollBar,
            "Slider"  => ControlType::Slider,
            "Spinner"  => ControlType::Spinner,
            "StatusBar"  => ControlType::StatusBar,
            "Tab"  => ControlType::Tab,
            "TabItem"  => ControlType::TabItem,
            "Text"  => ControlType::Text,
            "ToolBar"  => ControlType::ToolBar,
            "ToolTip"  => ControlType::ToolTip,
            "Tree"  => ControlType::Tree,
            "TreeItem"  => ControlType::TreeItem,
            "Custom"  => ControlType::Custom,
            "Group"  => ControlType::Group,
            "Thumb"  => ControlType::Thumb,
            "DataGrid"  => ControlType::DataGrid,
            "DataItem"  => ControlType::DataItem,
            "Document"  => ControlType::Document,
            "SplitButton"  => ControlType::SplitButton,
            "Window"  => ControlType::Window,
            "Pane"  => ControlType::Pane,
            "Header"  => ControlType::Header,
            "HeaderItem"  => ControlType::HeaderItem,
            "Table"  => ControlType::Table,
            "TitleBar"  => ControlType::TitleBar,
            "Separator"  => ControlType::Separator,
            "SemanticZoom"  => ControlType::SemanticZoom,
            "AppBar"  => ControlType::AppBar,
            _ => {
                log::warn!("Unknown control type '{}', defaulting to Custom", item);
                ControlType::Custom
            }
        };
        
        log::trace!("Converted control type '{}' to {:?}", item, control_type);
        control_type
    }
}

enum FindResult {
    FoundSingle(UIElement),
    FoundMultiple(Vec<UIElement>),
    NotFound,
}

impl std::fmt::Debug for FindResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindResult::FoundSingle(element) => {
                let name = element.get_name().unwrap_or_default();
                write!(f, "FoundSingle(name: {})", name)
            },
            FindResult::FoundMultiple(elements) => {
                write!(f, "FoundMultiple({} elements)", elements.len())
            },
            FindResult::NotFound => write!(f, "NotFound"),
        }
    }
}

pub fn get_element_by_xpath(xpath: String) -> Option<Element> {
    let _timer = PerformanceTimer::new("get_element_by_xpath");
    log_uiauto_operation!(log::Level::Info, "FIND_BY_XPATH", 
                         &format!("xpath_length={}", xpath.len()), 
                         "Starting element search by XPath");

    // Returns the Windows UI Automation API UI element of the window at the given xpath. As an xpath
    // is a string representation of the UI element, it is not a valid xpath in the XML sense.
    // The search is following a three step approach:
    // 1. A UI element is searched by its exact xpath.
    // 2. If the xpath does not provide a unique way to identify an elemt, the element is 
    //     searched for in the entire UI sub-tree.
    //     2.1. If there is a single matching element, this element is returned (irrespective if the xpath is a 100% match).
    //     2.2. If there are multiple matching elements, each found element is checked if the xpath
    //         matches and if a matching xpath is found the respective element is returned.
    // 3. if no matching element is found, None is returned.

    let mut input = xpath.as_str();
    let path_to_element: Vec<XpathElement<'_>>;
    let mut search_depth = 10; 
    
    log::debug!("Parsing XPath into element path");
    if let Ok(path_returned) = get_path_to_element(&mut input) {
        path_to_element = path_returned;
        log::debug!("Successfully parsed {} elements from XPath", path_to_element.len());
        
        for (i, element) in path_to_element.iter().enumerate() {
            log::debug!("Element {}: control_type={}, name={:?}, classname={:?}, automationid={:?}", 
                       i + 1, element.control_type, element.name, element.classname, element.automationid);
        }
    } else {
        log_uiauto_operation!(log::Level::Error, "FIND_BY_XPATH", "parse_failed", 
                             "Failed to parse XPath into element path");
        return None;
    }
    
    let uia = UIAutomation::new().unwrap();
    let mut root = uia.get_root_element().unwrap();
    
    log::debug!("Starting element traversal from root element");
    
    'outer: for (element_index, element) in path_to_element.iter().enumerate() {
        log_uiauto_operation!(log::Level::Debug, "TRAVERSE", 
                             &format!("element={}/{}", element_index + 1, path_to_element.len()), 
                             "Searching for element: control_type={}, attributes={}", 
                             element.control_type, element.attribute_count);
        
        let found = get_next_element(root.clone(), &element.clone(), search_depth);
        
        log::debug!("Search result for element {}: {:?}", element_index + 1, found);
        
        match found {
            FindResult::FoundSingle(found_element) => {
                log_uiauto_operation!(log::Level::Debug, "TRAVERSE", 
                                     &format!("element={}/{}", element_index + 1, path_to_element.len()), 
                                     "Found single matching element: {}", 
                                     found_element.get_name().unwrap_or_default());
                root = found_element;
            },
            FindResult::FoundMultiple(found_elements) => {
                log_uiauto_operation!(log::Level::Warn, "TRAVERSE", 
                                     &format!("element={}/{}", element_index + 1, path_to_element.len()), 
                                     "Found {} matching elements - trying lucky punch strategy", 
                                     found_elements.len());
                
                // trying the lucky punch and just search the target element (i.e. the last one in the xpath)
                search_depth = 99;
                let final_element = path_to_element.last().unwrap();
                
                log::debug!("Attempting lucky punch with final element: {:?}", final_element);
                let found = get_next_element(root.clone(), &final_element.clone(), search_depth);
                
                match found {
                    FindResult::FoundSingle(found_element) => {
                        log_uiauto_operation!(log::Level::Info, "TRAVERSE", "lucky_punch_success", 
                                             "Lucky punch successful - found target element: {}", 
                                             found_element.get_name().unwrap_or_default());
                        root = found_element;
                        break; // Exit the loop after finding the target element
                    },
                    FindResult::FoundMultiple(found_elements) => {
                        log_uiauto_operation!(log::Level::Warn, "TRAVERSE", "xpath_validation", 
                                             "Found {} candidates - validating by xpath generation", 
                                             found_elements.len());
                        
                        // loop through the found elements and construct a new xpath for each element
                        // and check if the xpath matches the target element
                        for (candidate_index, found_element) in found_elements.iter().enumerate() {
                            log::debug!("Validating candidate {} of {}", candidate_index + 1, found_elements.len());
                            
                            if let Ok(optional_point) = found_element.get_clickable_point() {
                                let point = optional_point.unwrap_or_default();
                                log::debug!("Candidate {} clickable point: ({}, {})", 
                                           candidate_index + 1, point.get_x(), point.get_y());
                                
                                let xpath_candidate = generate_xpath(point.get_x(), point.get_y());
                                
                                if xpath_candidate == xpath {
                                    log_uiauto_operation!(log::Level::Info, "TRAVERSE", "xpath_match", 
                                                         "Found matching element by xpath validation: {}", 
                                                         found_element.get_name().unwrap_or_default());
                                    root = found_element.clone();
                                    break 'outer; // Exit the inner and outer loop after finding the target element
                                } else {
                                    log::debug!("Candidate {} xpath mismatch. Expected: {}, Got: {}", 
                                               candidate_index + 1, 
                                               if xpath.len() > 100 { &xpath[..100] } else { &xpath },
                                               if xpath_candidate.len() > 100 { &xpath_candidate[..100] } else { &xpath_candidate });
                                }
                            } else {
                                log::debug!("Failed to get clickable point for candidate {}", candidate_index + 1);
                            }
                        }
                        
                        log_uiauto_operation!(log::Level::Error, "TRAVERSE", "no_match", 
                                             "No matching element found after xpath validation");
                        return None; // Return None if we find multiple elements again
                        
                    },
                    FindResult::NotFound => {
                        log_uiauto_operation!(log::Level::Error, "TRAVERSE", "final_not_found", 
                                             "Final element not found during lucky punch attempt");
                        return None;
                    }
                } 
            },
            FindResult::NotFound => {
                log_uiauto_operation!(log::Level::Error, "TRAVERSE", 
                                     &format!("element={}/{}", element_index + 1, path_to_element.len()), 
                                     "Element not found: control_type={}", element.control_type);
                return None;
            }
        }
    }

    // If we reach here, we have found the element
    let name = root.get_name().unwrap_or("".to_string());
    let xpath = "".to_string(); // Placeholder for the xpath, as we don't have a function to generate it from the element
    let handle: isize = root.get_native_window_handle().unwrap_or_default().into();
    let runtimeid: Vec<i32> = root.get_runtime_id().unwrap_or_default();
    let bounding_rectangle = root.get_bounding_rectangle().unwrap_or_default();
    let (left, top, right, bottom) =(
        bounding_rectangle.get_left(),
        bounding_rectangle.get_top(),
        bounding_rectangle.get_right(),
        bounding_rectangle.get_bottom(),
    );
    
    let element = Element::new(name.clone(), xpath, handle, runtimeid.clone(), (left, top, right, bottom));
    
    log_uiauto_operation!(log::Level::Info, "FIND_BY_XPATH", "success", 
                         "Successfully found element: name='{}', handle={}, runtime_id={:?}, bounds=({},{},{},{})", 
                         name, handle, runtimeid, left, top, right, bottom);
    
    Some(element)
}

fn get_next_element(root: UIElement, element: &XpathElement<'_>, depth: u32 ) -> FindResult {
    let _timer = PerformanceTimer::new("get_next_element");
    log_uiauto_operation!(log::Level::Debug, "SEARCH", 
                         &format!("control_type={}, depth={}", element.control_type, depth), 
                         "Searching for element with {} attributes", element.attribute_count);

    let uia = UIAutomation::new().unwrap();
    let matcher = uia.create_matcher().from(root.clone()).depth(depth);

    let control_type = ControlType::from_str(element.control_type);
    let matcher = matcher.control_type(control_type);
    log::debug!("Added control type filter: {:?}", control_type);

    let matcher = if element.name.is_some() {
        log::debug!("Adding name filter: {}", element.name.unwrap());
        matcher.name(element.name.unwrap())
    } else {
        log::trace!("No name filter specified");
        matcher
    };
    
    let matcher = if element.classname.is_some() {
        log::debug!("Adding classname filter: {}", element.classname.unwrap());
        matcher.classname(element.classname.unwrap())
    } else {
        log::trace!("No classname filter specified");
        matcher
    };

    // TODO: add a filter function for automationid
    // let matcher = if element.automationid.is_some() {matcher.automationid(element.automationid)} else {matcher};
    
    if element.automationid.is_some() {
        log::debug!("AutomationId filter requested but not implemented: {}", element.automationid.unwrap());
    }

    log::debug!("Executing element search with configured filters");
    
    if let Ok(found_elements) = matcher.find_all() { 
        log::debug!("Search completed. Found {} elements", found_elements.len());
        
        if found_elements.len() == 1 {
            let element_name = found_elements[0].get_name().unwrap_or_default();
            log_uiauto_operation!(log::Level::Debug, "SEARCH", "single_match", 
                                 "Found exactly one element: {}", element_name);
            return FindResult::FoundSingle(found_elements[0].clone());
        } else if found_elements.len() > 1 {
            log_uiauto_operation!(log::Level::Debug, "SEARCH", "multiple_matches", 
                                 "Found {} elements:", found_elements.len());
            
            for (i, elem) in found_elements.iter().enumerate() {
                let elem_name = elem.get_name().unwrap_or_default();
                let elem_classname = elem.get_classname().unwrap_or_default();
                log::debug!("  Element {}: name='{}', classname='{}'", i + 1, elem_name, elem_classname);
            }
            
            return FindResult::FoundMultiple(found_elements);
        } else {
            log_uiauto_operation!(log::Level::Debug, "SEARCH", "no_matches", 
                                 "No elements found matching the criteria");
            return FindResult::NotFound;
        }
    } else {
        log_uiauto_operation!(log::Level::Error, "SEARCH", "search_error", 
                             "Error occurred during element search");
        return FindResult::NotFound;
    }
}

pub fn get_ui_element_by_xpath(xpath: String) -> Option<UIElement> {
    let _timer = PerformanceTimer::new("get_ui_element_by_xpath");
    log_uiauto_operation!(log::Level::Info, "GET_UI_ELEMENT", 
                         &format!("xpath_length={}", xpath.len()), 
                         "Converting Bromium Element to UIElement");

    let ui_elem = get_element_by_xpath(xpath.clone());
    if ui_elem.is_none() {
        log_uiauto_operation!(log::Level::Error, "GET_UI_ELEMENT", "element_not_found", 
                             "Failed to find element by xpath");
        return None;
    }
    let element = ui_elem.unwrap();

    let runtime_id = element.get_runtime_id();
    log::debug!("Found element with runtime_id: {:?}", runtime_id);

    get_ui_element_by_runtimeid(runtime_id)
}

struct RuntimeIdFilter(Vec<i32>);

impl uiautomation::filters::MatcherFilter for RuntimeIdFilter {
    fn judge(&self, element: &UIElement) -> uiautomation::Result<bool> {
        // self is the element we are looking for
        // element is the element we are checking against
        let id = element.get_runtime_id()?;
        let matches = id == self.0;
        if matches {
            log::trace!("Runtime ID match found: {:?}", id);
        }
        Ok(matches)
    }
}

pub fn get_ui_element_by_runtimeid(runtime_id: Vec<i32>) -> Option<UIElement> {
    let _timer = PerformanceTimer::new("get_ui_element_by_runtimeid");
    log_uiauto_operation!(log::Level::Debug, "FIND_BY_RUNTIME_ID", 
                         &format!("runtime_id={:?}", runtime_id), 
                         "Searching for element by runtime ID");

    let automation = UIAutomation::new().unwrap();
    let matcher = automation.create_matcher()
        .timeout(0)
        .filter(Box::new(RuntimeIdFilter(runtime_id.clone())))
        .depth(99);
    
    log::debug!("Executing runtime ID search with depth 99");
    let element = matcher.find_first();
    
    match element {
        Ok(e) => {
            let element_name = e.get_name().unwrap_or_default();
            log_uiauto_operation!(log::Level::Info, "FIND_BY_RUNTIME_ID", "success", 
                                 "Found element by runtime ID: name='{}', runtime_id={:?}", 
                                 element_name, runtime_id);
            Some(e)
        },
        Err(e) => {
            log_uiauto_operation!(log::Level::Error, "FIND_BY_RUNTIME_ID", "error", 
                                 "Failed to find element by runtime ID {:?}: {}", runtime_id, e);
            None
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_element_by_xpath() {
        // Initialize logging for tests
        let _ = env_logger::builder().is_test(true).try_init();
        
        let xpath = r##"/Pane[@ClassName=\"#32769\"][@Name=\"Desktop 1\"]/Pane[@ClassName=\"Shell_TrayWnd\"][@Name=\"Taskleiste\"]/Pane[@ClassName=\"Windows.UI.Input.InputSite.WindowClass\"]/Pane[@ClassName=\"Taskbar.TaskbarFrameAutomationPeer\"][@AutomationId=\"TaskbarFrame\"]/Button[@Name=\"Start\"][@AutomationId=\"StartButton\"]"##; 
        
        log::info!("Starting test with XPath: {}", xpath);
        
        let element = get_element_by_xpath(xpath.to_string());
        assert!(element.is_some(), "Element not found for XPath: {}", xpath);
        
        if let Some(elem) = element {
            log::info!("Test successful - found element: {}", elem.get_name());
        }
    }
}