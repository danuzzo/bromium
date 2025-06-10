// things required for the XPath generation
use crate::bindings;
use crate::logging::PerformanceTimer;
use crate::{log_xpath_operation};
use winapi::um::winuser::SetProcessDPIAware;

// things required for the XPath parsing
use winnow::{
    ascii::alpha1,
    combinator::{delimited, separated_pair},
    prelude::*,
    token::take_until,
    Result,
};

// region: XPath generation

// Extract runtime ID from the XPath
#[allow(dead_code)]
fn extract_runtime_id(xpath: &str) -> Option<String> {
    log::debug!("Extracting runtime ID from XPath: {}", 
               if xpath.len() > 100 { &xpath[..100] } else { xpath });
    
    let pattern = r#"\[RuntimeId="([^"]+)"\]"#;
    let re = regex::Regex::new(pattern).ok()?;
    
    for (line_num, line) in xpath.lines().enumerate() {
        if let Some(captures) = re.captures(line) {
            if let Some(runtime_id) = captures.get(1) {
                let id = runtime_id.as_str().to_string();
                log::debug!("Found runtime ID '{}' at line {}", id, line_num + 1);
                return Some(id);
            }
        }
    }
    
    log::debug!("No runtime ID found in XPath");
    None
}

// Function to simplify XPath by removing extra attributes and formatting correctly
#[allow(dead_code)]
fn simplify_xpath(xpath: &str) -> String {
    log::debug!("Simplifying XPath with {} characters", xpath.len());
    
    let lines: Vec<&str> = xpath.split('\n').filter(|line| !line.is_empty()).collect();
    let mut elements = Vec::new();
    
    log::debug!("Processing {} non-empty lines from XPath", lines.len());
    
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        
        log::trace!("Processing line {}: {}", index + 1, line);
        
        // Extract the tag name (everything before the first '[')
        let tag_end = line.find('[').unwrap_or(line.len());
        let tag = &line[1..tag_end]; // Skip the leading '/'
        
        let mut element = format!("/{}", tag);
        log::trace!("Extracted tag: {}", tag);
        
        // Helper function to extract attribute value and format it with escaped quotes
        let extract_attr = |attr_name: &str, line: &str| -> String {
            let attr_prefix = format!("[{}=\"", attr_name);
            if let Some(start_idx) = line.find(&attr_prefix) {
                let value_start = start_idx + attr_prefix.len();
                if let Some(end_idx) = line[value_start..].find("\"]") {
                    let value = &line[value_start..value_start + end_idx];
                    log::trace!("Found attribute {}={}", attr_name, value);
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
        log::trace!("Built element: {}", elements.last().unwrap());
    }
    
    // Reverse the elements to go from root to specific element
    elements.reverse();
    
    // Join all elements into a single XPath string
    let result = elements.join("");
    log::debug!("Simplified XPath result: {}", result);
    result
}

// Function that tries to match the original C++ XPath format
fn match_original_format(xpath: &str) -> String {
    let _timer = PerformanceTimer::new("match_original_format");
    log_xpath_operation!(log::Level::Debug, "FORMAT", 
                        &format!("input_length={}", xpath.len()), 
                        "Matching original C++ XPath format");
    
    let lines: Vec<&str> = xpath.split('\n').filter(|line| !line.is_empty()).collect();
    let mut elements = Vec::new();
    
    log::debug!("Processing {} lines for original format matching", lines.len());
    
    for (line_index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        
        log::trace!("Processing line {} of {}: {}", line_index + 1, lines.len(), 
                   if line.len() > 150 { &line[..150] } else { line });
        
        // Extract the tag name (everything before the first '[')
        let tag_end = line.find('[').unwrap_or(line.len());
        let tag = &line[1..tag_end]; // Skip the leading '/'
        
        let mut element = format!("/{}", tag);
        log::trace!("Extracted control type: {}", tag);
        
        // Helper function to extract attribute value and format it with escaped quotes
        let extract_attr = |attr_name: &str, line: &str| -> Option<String> {
            let attr_prefix = format!("[{}=\"", attr_name);
            if let Some(start_idx) = line.find(&attr_prefix) {
                let value_start = start_idx + attr_prefix.len();
                if let Some(end_idx) = line[value_start..].find("\"]") {
                    let value = &line[value_start..value_start + end_idx];
                    // Skip empty attributes
                    if value.is_empty() {
                        log::trace!("Skipping empty {} attribute", attr_name);
                        return None;
                    }
                    log::trace!("Extracted {}=\"{}\"", attr_name, value);
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
                log::trace!("Added ClassName for {} element", tag);
            }
        } else if tag == "Group" {
            // For Group, only include ClassName if it's "LandmarkTarget"
            if let Some(class_value) = get_attr_value("ClassName", line) {
                if class_value == "LandmarkTarget" {
                    if let Some(class_attr) = extract_attr("ClassName", line) {
                        element.push_str(&class_attr);
                        log::trace!("Added LandmarkTarget ClassName for Group element");
                    }
                } else {
                    log::trace!("Skipping ClassName '{}' for Group element", class_value);
                }
            }
        }
        // For other elements like Button and Custom, don't include ClassName
        
        // Add Name attribute (if non-empty)
        if let Some(name_attr) = extract_attr("Name", line) {
            element.push_str(&name_attr);
            log::trace!("Added Name attribute to {} element", tag);
        }
        
        // Add AutomationId attribute (if non-empty)
        if let Some(id_attr) = extract_attr("AutomationId", line) {
            element.push_str(&id_attr);
            log::trace!("Added AutomationId attribute to {} element", tag);
        }
        
        elements.push(element);
        log::debug!("Built element {}: {}", line_index + 1, elements.last().unwrap());
    }
    
    // Reverse the elements to go from root to specific element
    elements.reverse();
    log::debug!("Reversed {} elements to create root-to-target path", elements.len());
    
    // Join all elements into a single XPath string
    let result = elements.join("");
    log_xpath_operation!(log::Level::Debug, "FORMAT", 
                        &format!("output_length={}", result.len()), 
                        "XPath formatting completed: {}", 
                        if result.len() > 200 { &result[..200] } else { &result });
    result
}

pub fn generate_xpath(x: i32, y: i32) -> String {
    let _timer = PerformanceTimer::new("generate_xpath");
    log_xpath_operation!(log::Level::Info, "GENERATE", 
                        &format!("coordinates=({}, {})", x, y), 
                        "Starting XPath generation");

    let mut original_format = String::from("No XPath found");

    unsafe {
        log::debug!("Setting process DPI awareness");
        // Make the application DPI-aware (as done in the original C# app)
        SetProcessDPIAware();

        log::debug!("Initializing UI Tree Walk");
        // Initialize UI Automation directly through our C++ bindings
        bindings::InitUiTreeWalk();

        // Normal mode - get XPath
        let buffer_size = 4096 * 4; // BUFFERSIZE from UiTreeWalk.h
        let mut path_buffer = vec![0u16; buffer_size];
        
        log::debug!("Calling GetUiXPath with buffer size: {}", buffer_size);
        let path_len = bindings::GetUiXPath(x, y, path_buffer.as_mut_ptr(), path_buffer.len() as i32);
        
        log_xpath_operation!(log::Level::Debug, "GENERATE", 
                            &format!("coordinates=({}, {})", x, y), 
                            "Raw XPath length: {}", path_len);
        
        if path_len > 0 {
            let path = String::from_utf16_lossy(&path_buffer[0..path_len as usize]);
            
            log::debug!("Raw XPath received from C++ binding:");
            log::debug!("Length: {} characters", path.len());
            
            // Log the raw path in chunks to avoid overwhelming logs
            for (i, chunk) in path.lines().enumerate() {
                if !chunk.is_empty() {
                    log::trace!("Raw XPath line {}: {}", i + 1, chunk);
                }
            }
            
            log::debug!("Converting to original C++ format");
            original_format = match_original_format(&path);
            
            log_xpath_operation!(log::Level::Info, "GENERATE", 
                                &format!("coordinates=({}, {})", x, y), 
                                "XPath generation successful. Final XPath: {}", original_format);
        } else {
            log_xpath_operation!(log::Level::Warn, "GENERATE", 
                                &format!("coordinates=({}, {})", x, y), 
                                "No XPath generated - path length is 0");
        }
        
        log::debug!("Cleaning up UI Tree Walk");
        // Clean up
        bindings::UnInitUiTreeWalk();
    }   

    // Return the generated XPath
    original_format
}

// endregion: XPath generation

// region: XPath parsing
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct XpathElement<'a> {
    pub control_type: &'a str,
    pub classname: Option<&'a str>,
    pub name: Option<&'a str>,
    pub automationid: Option<&'a str>,
    pub attribute_count: usize,
}

impl Default for XpathElement<'_> {
    fn default() -> Self {
        XpathElement {
            control_type: "",
            classname: None,
            name: None,
            automationid: None,
            attribute_count: 0,
        }
    }
}

fn parse_at_identifier<'a>(input: &mut &'a str) -> Result<&'a str> {
    let (_, identifier) = ("@", alpha1).parse_next(input)?;
    log::trace!("Parsed attribute identifier: {}", identifier);
    Ok(identifier)
}

fn parse_element_control_type<'a>(input: &mut &'a str) -> Result<&'a str> {
    let control_type = alpha1.parse_next(input)?;
    log::trace!("Parsed control type: {}", control_type);
    Ok(control_type)
}

fn parse_attribute_value<'a>(input: &mut &'a str) -> Result<&'a str> {
    let value = delimited(
        "\\\"",
        take_until(1.., "\\\""),
        "\\\"",
    ).parse_next(input)?;
    log::trace!("Parsed attribute value: {}", value);
    Ok(value)
}

fn parse_attribute<'a>(input: &mut &'a str) -> Result<Attribute<'a>> {
    let input_preview = if input.len() > 50 { &input[..50] } else { *input };
    log::trace!("Parsing attribute from: {}", input_preview);
    
    let (key, value) = delimited(
        "[",
        separated_pair(
            parse_at_identifier,
            "=",
            parse_attribute_value
        ),
        "]",
    ).parse_next(input)?;
    
    log::trace!("Successfully parsed attribute: {}={}", key, value);
    Ok(Attribute { key, value })
}

fn parse_element<'a>(input: &mut &'a str) -> Result<XpathElement<'a>> {
    let input_preview = if input.len() > 100 { &input[..100] } else { *input };
    log::debug!("Parsing element from: {}", input_preview);
    
    let control_type = parse_element_control_type(input)?;    
    let mut attribute_count = 0;
    let mut classname: Option<&str> = None;
    let mut name: Option<&str> = None;
    let mut automationid: Option<&str> = None;
    
    log::debug!("Parsing attributes for control type: {}", control_type);
    
    while let Ok(attr) = parse_attribute(input) {
        match attr.key {
            "ClassName" => {
                log::debug!("Found ClassName: {}", attr.value);
                classname = Some(attr.value);
                attribute_count += 1;
            },
            "Name" => {
                log::debug!("Found Name: {}", attr.value);
                name = Some(attr.value);
                attribute_count += 1;
            },
            "AutomationId" => {
                log::debug!("Found AutomationId: {}", attr.value);
                automationid = Some(attr.value);
                attribute_count += 1;
            },
            _ => {
                log::trace!("Skipping unsupported attribute: {}", attr.key);
            }
        }
    }
    
    let element = XpathElement { control_type, classname, name, automationid, attribute_count };
    log::debug!("Parsed element: {:?}", element);
    Ok(element)
}

pub fn get_path_to_element<'a>(input: &mut &'a str) -> Result<Vec<XpathElement<'a>>> {
    let _timer = PerformanceTimer::new("get_path_to_element");
    log_xpath_operation!(log::Level::Info, "PARSE", 
                        &format!("input_length={}", input.len()), 
                        "Starting XPath parsing");

    let mut path_to_element: Vec<XpathElement<'a>> = Vec::new();
    
    log::debug!("Parsing XPath: {}", input);
    
    // Work directly with string slices to avoid lifetime issues
    let xpath_content = *input;
    let parts: Vec<&str> = xpath_content.split('/').skip(2).collect();
    
    log::debug!("XPath tree structure: {:#?}", parts);
    log::debug!("Found {} elements in XPath tree", parts.len());

    for (index, element_str) in parts.iter().enumerate() {
        log::debug!("Parsing element {} of {}: {}", index + 1, parts.len(), element_str);
        
        let mut element_input = *element_str;
        match parse_element(&mut element_input) {
            Ok(parsed_element) => {
                log_xpath_operation!(log::Level::Debug, "PARSE", 
                                    &format!("element_index={}", index + 1), 
                                    "Successfully parsed element: control_type={}, attributes={}", 
                                    parsed_element.control_type, parsed_element.attribute_count);
                path_to_element.push(parsed_element);
            },
            Err(e) => {
                log_xpath_operation!(log::Level::Error, "PARSE", 
                                    &format!("element_index={}", index + 1), 
                                    "Failed to parse element '{}': {}", element_str, e);
                return Err(e);
            }
        }
    }
    
    log_xpath_operation!(log::Level::Info, "PARSE", 
                        &format!("elements_count={}", path_to_element.len()), 
                        "XPath parsing completed successfully");
    Ok(path_to_element)
}
// endregion: XPath parsing