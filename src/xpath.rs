use crate::bindings;
use winapi::um::winuser::SetProcessDPIAware;

// Extract runtime ID from the XPath
#[allow(dead_code)]
fn extract_runtime_id(xpath: &str) -> Option<String> {
    let pattern = r#"\[RuntimeId="([^"]+)"\]"#;
    let re = regex::Regex::new(pattern).ok()?;
    
    for line in xpath.lines() {
        if let Some(captures) = re.captures(line) {
            if let Some(runtime_id) = captures.get(1) {
                return Some(runtime_id.as_str().to_string());
            }
        }
    }
    None
}




// Function to simplify XPath by removing extra attributes and formatting correctly
#[allow(dead_code)]
fn simplify_xpath(xpath: &str) -> String {
    let lines: Vec<&str> = xpath.split('\n').filter(|line| !line.is_empty()).collect();
    let mut elements = Vec::new();
    
    for line in lines {
        if line.is_empty() {
            continue;
        }
        
        // Extract the tag name (everything before the first '[')
        let tag_end = line.find('[').unwrap_or(line.len());
        let tag = &line[1..tag_end]; // Skip the leading '/'
        
        let mut element = format!("/{}", tag);
        
        // Helper function to extract attribute value and format it with escaped quotes
        let extract_attr = |attr_name: &str, line: &str| -> String {
            let attr_prefix = format!("[{}=\"", attr_name);
            if let Some(start_idx) = line.find(&attr_prefix) {
                let value_start = start_idx + attr_prefix.len();
                if let Some(end_idx) = line[value_start..].find("\"]") {
                    let value = &line[value_start..value_start + end_idx];
                    return format!("[@{}=\\\"{}\\\"]", attr_name, value);
                }
            }
            String::new()
        };
        
        // Add ClassName attribute
        let class_attr = extract_attr("ClassName", line);
        if !class_attr.is_empty() {
            element.push_str(&class_attr);
        }
        
        // Add Name attribute
        let name_attr = extract_attr("Name", line);
        if !name_attr.is_empty() {
            element.push_str(&name_attr);
        }
        
        // Add AutomationId attribute
        let id_attr = extract_attr("AutomationId", line);
        if !id_attr.is_empty() {
            element.push_str(&id_attr);
        }
        
        elements.push(element);
    }
    
    // Reverse the elements to go from root to specific element
    elements.reverse();
    
    // Join all elements into a single XPath string
    elements.join("")
}

// Function that tries to match the original C++ XPath format
fn match_original_format(xpath: &str) -> String {
    let lines: Vec<&str> = xpath.split('\n').filter(|line| !line.is_empty()).collect();
    let mut elements = Vec::new();
    
    for line in lines {
        if line.is_empty() {
            continue;
        }
        
        // Extract the tag name (everything before the first '[')
        let tag_end = line.find('[').unwrap_or(line.len());
        let tag = &line[1..tag_end]; // Skip the leading '/'
        
        let mut element = format!("/{}", tag);
        
        // Helper function to extract attribute value and format it with escaped quotes
        let extract_attr = |attr_name: &str, line: &str| -> Option<String> {
            let attr_prefix = format!("[{}=\"", attr_name);
            if let Some(start_idx) = line.find(&attr_prefix) {
                let value_start = start_idx + attr_prefix.len();
                if let Some(end_idx) = line[value_start..].find("\"]") {
                    let value = &line[value_start..value_start + end_idx];
                    // Skip empty attributes
                    if value.is_empty() {
                        return None;
                    }
                    return Some(format!("[@{}=\\\"{}\\\"]", attr_name, value));
                }
            }
            None
        };
        
        // Helper function to get just the attribute value
        let get_attr_value = |attr_name: &str, line: &str| -> Option<String> {
            let attr_prefix = format!("[{}=\"", attr_name);
            if let Some(start_idx) = line.find(&attr_prefix) {
                let value_start = start_idx + attr_prefix.len();
                if let Some(end_idx) = line[value_start..].find("\"]") {
                    let value = &line[value_start..value_start + end_idx];
                    if !value.is_empty() {
                        return Some(value.to_string());
                    }
                }
            }
            None
        };
        
        // More complex logic to match original C++ behavior
        if tag == "Pane" || tag == "Window" {
            // Always include ClassName for Pane and Window
            if let Some(class_attr) = extract_attr("ClassName", line) {
                element.push_str(&class_attr);
            }
        } else if tag == "Group" {
            // For Group, only include ClassName if it's "LandmarkTarget"
            if let Some(class_value) = get_attr_value("ClassName", line) {
                if class_value == "LandmarkTarget" {
                    if let Some(class_attr) = extract_attr("ClassName", line) {
                        element.push_str(&class_attr);
                    }
                }
                // Skip ClassName for Group elements with other classes like "NamedContainerAutomationPeer"
            }
        }
        // For other elements like Button and Custom, don't include ClassName
        
        // Add Name attribute (if non-empty)
        if let Some(name_attr) = extract_attr("Name", line) {
            element.push_str(&name_attr);
        }
        
        // Add AutomationId attribute (if non-empty)
        if let Some(id_attr) = extract_attr("AutomationId", line) {
            element.push_str(&id_attr);
        }
        
        elements.push(element);
    }
    
    // Reverse the elements to go from root to specific element
    elements.reverse();
    
    // Join all elements into a single XPath string
    elements.join("")
}


pub fn generate_xpath(x: i32, y: i32) -> String {

    let mut original_format = String::from("No XPath found");

    unsafe {
        // Make the application DPI-aware (as done in the original C# app)
        SetProcessDPIAware();

        // Initialize UI Automation directly through our C++ bindings
        bindings::InitUiTreeWalk();

        // Normal mode - get XPath
        let mut path_buffer = vec![0u16; 4096 * 4]; // BUFFERSIZE from UiTreeWalk.h
        let path_len = // unsafe {
            bindings::GetUiXPath(x, y, path_buffer.as_mut_ptr(), path_buffer.len() as i32);
        //};

        if path_len > 0 {
            let path = String::from_utf16_lossy(&path_buffer[0..path_len as usize]);
            
            
            // println!("\n============= Raw Element XPath =============");
            // println!("{}", path);
            
            // // Show simplified version that includes all non-empty attributes
            // let simplified_path = simplify_xpath(&path);
            // println!("\n----- Simplified XPath (all attributes) -----");
            // println!("{}", simplified_path);
            
            // // Show a version that tries to match original C++ implementation
            // let original_format = match_original_format(&path);
            // println!("\n----- Original C++ Style XPath Format -----");
            // println!("{}", original_format);
            // println!("========================================\n");
            
            original_format = match_original_format(&path);
        }
        // Clean up
        bindings::UnInitUiTreeWalk();
        

    }   

    // Return the generated XPath
    original_format


}