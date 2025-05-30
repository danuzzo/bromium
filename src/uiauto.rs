use crate::windriver::Element;
use crate::xpath::generate_xpath;
use crate::xpath::get_path_to_element;
use crate::xpath::XpathElement;
use uiautomation::{controls::ControlType, UIAutomation, UIElement};

trait ConvertToControlType {
    fn from_str(item: &str) -> ControlType;
}

impl ConvertToControlType for ControlType {
    fn from_str(item: &str) -> Self {
        match item {
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
            _ => ControlType::Custom, // Default case
        }
    }
}

enum FindResult {
    FoundSingle(UIElement),
    FoundMultiple(Vec<UIElement>),
    NotFound,
}


pub fn get_element_by_xpath(xpath: String) -> Option<Element> {
    
    let mut input = xpath.as_str();
    let path_to_element: Vec<XpathElement<'_>>;
    let mut search_depth = 10; 
    
    if let Ok(path_returned) = get_path_to_element(&mut input) {
        // println!("Path to element: {:?}", path_to_element);
        path_to_element = path_returned;
    } else {
        println!("Failed to get path to element.");
        return None;
    }
    
    let uia = UIAutomation::new().unwrap();
    let mut root = uia.get_root_element().unwrap();
    'outer: for element in &path_to_element {
        println!("Looking for Element: {:?}", element);            
        let found = get_next_element(root.clone(), &element.clone(), search_depth);
        match found {
            FindResult::FoundSingle(found_element) => {
                println!("Element found: {:?}", found_element);
                root = found_element;
            },
            FindResult::FoundMultiple(found_elements) => {
                println!("Found multiple elements: {:?}", found_elements);
                // trying the lucky punch and just search the target element (i.e. the last one in the xpath)
                search_depth = 50;
                let final_element = path_to_element.last().unwrap();
                let found = get_next_element(root.clone(), &final_element.clone(), search_depth);
                match found {
                    FindResult::FoundSingle(found_element) => {
                        println!("Element found: {:?}", found_element);
                        root = found_element;
                        break; // Exit the loop after finding the target element
                    },
                    FindResult::FoundMultiple(found_elements) => {
                        println!("Found again multiple elements: {:?}", found_elements);
                        // loop through the found elements and construct a new xpath for each element
                        // and check if the xpath matches the target element
                        for found_element in found_elements {
                            if let Ok(optional_point) = found_element.get_clickable_point() {
                                let point = optional_point.unwrap_or_default();
                                println!("Found element at: {:?}", point);
                                let xpath_candidate = generate_xpath(point.get_x(), point.get_y());
                                if xpath_candidate == xpath {
                                    println!("Found target element: {:?}", found_element);
                                    root = found_element;
                                    break 'outer; // Exit the inner and outer loop after finding the target element
                                } else {
                                    println!("Found element but not matching xpath: {:?}", found_element);
                                    //skip this element
                                }
                            } else {
                                println!("Failed to get clickable point for element: {:?}", found_element);
                                //skip this element
                            }
                        }
                        
                        println!("No matching element found for xpath: {:?}", xpath);
                        return None; // Return None if we find multiple elements again
                        
                    },
                    FindResult::NotFound => {
                        println!("Element not found: {:?}", final_element);
                        return None;
                    }
                } 
            },
            FindResult::NotFound => {
                println!("Element not found: {:?}", element);
                return None;
            }
        }
    }



    // If we reach here, we have found the element
    let name = root.get_name().unwrap_or("".to_string());
    let xpath = "".to_string(); // Placeholder for the xpath, as we don't have a function to generate it from the element
    let handle: isize = root.get_native_window_handle().unwrap_or_default().into();
    let runtimeid: Vec<i32> = root.get_runtime_id().unwrap_or_default();
    let element = Element::new(name, xpath, handle, runtimeid);
    println!("Final Element: {:?}", element);
    Some(element)
}

fn get_next_element(root: UIElement, element: &XpathElement<'_>, depth: u32 ) -> FindResult {
    let uia = UIAutomation::new().unwrap();
    let matcher = uia.create_matcher().from(root).depth(depth);

    let control_type = ControlType::from_str(element.control_type);
    let matcher = matcher.control_type(control_type);

    let matcher = if element.name.is_some() {matcher.name(element.name.unwrap())} else {matcher};
    let matcher = if element.classname.is_some() {matcher.classname(element.classname.unwrap())} else {matcher};

    // TODO: add a filter function for automationid
    // let matcher = if element.automationid.is_some() {matcher.automationid(element.automationid)} else {matcher};
    // let matcher = matcher.filter_fn(
    //     Box::new(|e: &UIElement| {
    //         let framework_id = e.get_framework_id()?;
    //         let class_name = e.get_classname()?;
        
    //         Ok("Win32" == framework_id && class_name.starts_with("Shell"))
    //     }
    // ));

    println!("Matcher: {:?}", matcher);
    
    if let Ok(found_elements) = matcher.find_all() { 
        if found_elements.len() == 1 {
            // println!("Found exactly one element: {:?}", found_elements);
            return FindResult::FoundSingle(found_elements[0].clone());
        } else {
            // println!("Found multiple elements: {:?}", found_elements);
            return FindResult::FoundMultiple(found_elements);
        }
    } else {
        // println!("No elements found.");
        return FindResult::NotFound;
    }
    
}


pub fn get_ui_element_by_xpath(xpath: String) -> Option<UIElement> {


    let ui_elem = get_element_by_xpath(xpath.clone());
    if ui_elem.is_none() {
        return None;
    }
    let element = ui_elem.unwrap();

    let runtime_id = element.get_runtime_id();

    get_ui_element_by_runtimeid(runtime_id)

    
}



struct RuntimeIdFilter(Vec<i32>);

impl uiautomation::filters::MatcherFilter for RuntimeIdFilter {
    fn judge(&self, element: &UIElement) -> uiautomation::Result<bool> {
        let id = element.get_runtime_id()?;
        Ok(id == self.0)
    }
}


pub fn get_ui_element_by_runtimeid(runtime_id: Vec<i32>) -> Option<UIElement> {
    let automation = UIAutomation::new().unwrap();
    let matcher = automation.create_matcher().timeout(0).filter(Box::new(RuntimeIdFilter(runtime_id))).depth(2);
    let element = matcher.find_first();
    
    match element {
        Ok(e) => {
            println!("Element found: {:?}", e);
            Some(e)
        },
        Err(e) => {
            println!("Error finding element: {:?}", e);
            None
        }
    }
    
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_element_by_xpath() {
        let xpath = r##"/Pane[@ClassName=\"#32769\"][@Name=\"Desktop 1\"]/Pane[@ClassName=\"Shell_TrayWnd\"][@Name=\"Taskleiste\"]/Pane[@ClassName=\"Windows.UI.Input.InputSite.WindowClass\"]/Pane[@ClassName=\"Taskbar.TaskbarFrameAutomationPeer\"][@AutomationId=\"TaskbarFrame\"]/Button[@Name=\"Start\"][@AutomationId=\"StartButton\"]"##; 
        let element = get_element_by_xpath(xpath.to_string());
        assert!(element.is_some(), "Element not found for XPath: {} -> {}", xpath, element.unwrap().get_name());
    }
}