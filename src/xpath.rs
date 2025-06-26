// things required for the XPath generation
// use crate::bindings;
// use winapi::um::winuser::SetProcessDPIAware;

// things required for the XPath parsing
use winnow::{
    ascii::alpha1,
    combinator::{delimited, separated_pair},
    prelude::*,
    token::take_until,
    Result,
};

use crate::printfmt;



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
    // pub attributes: Vec<Attribute<'a>>,
    pub attribute_count: usize,
}

impl Default for XpathElement<'_> {
    fn default() -> Self {
        XpathElement {
            control_type: "",
            classname: None,
            name: None,
            automationid: None,
            // attributes: Vec::new(),
            attribute_count: 0,
        }
    }
}

fn parse_at_identifier<'a>(input: &mut &'a str) -> Result<&'a str> {
    let (_, identifier) = ("@", alpha1).parse_next(input)?;
    Ok(identifier)
}

fn parse_element_control_type<'a>(input: &mut &'a str) -> Result<&'a str> {
    alpha1.parse_next(input)
}

fn parse_attribute_value<'a>(input: &mut &'a str) -> Result<&'a str> {
    delimited(
        "\\\"",
        take_until(1.., "\\\""),  //take_till(1.., |c| c == '"'),
        "\\\"",
    ).parse_next(input)
}

fn parse_attribute<'a>(input: &mut &'a str) -> Result<Attribute<'a>> {
    let (key, value) = delimited(
        "[",
        separated_pair(
            parse_at_identifier,
            "=",
            parse_attribute_value
        ),
        "]",
    ).parse_next(input)?;
    
    Ok(Attribute { key, value })
}

fn parse_element<'a>(input: &mut &'a str) -> Result<XpathElement<'a>> {
    printfmt!("Parsing element: {}", input);
    let control_type = parse_element_control_type(input)?;    
    printfmt!("Control type: {}", control_type);
    let mut attribute_count = 0;
    let mut classname: Option<&str> = None;
    let mut name: Option<&str> = None;
    let mut automationid: Option<&str> = None;
    // let mut attributes = Vec::new();
    
    printfmt!("Parsing attributes for element: {}", input);
    while let Ok(attr) = parse_attribute(input) {
        printfmt!("Parsed attribute: {:?}", attr);
        // attributes.push(attr);
        match attr.key {
            "ClassName" => {
                // println!("ClassName: {}", attr.value);
                classname = Some(attr.value);
                attribute_count += 1;
            },
            "Name" => {
                // println!("Name: {}", attr.value);
                name = Some(attr.value);
                attribute_count += 1;
            },
            "AutomationId" => {
                // println!("AutomationId: {}", attr.value);
                automationid = Some(attr.value);
                attribute_count += 1;
            },
            _ => {}
        }
        
    }
    
    // let attribute_count = attributes.len();
    Ok(XpathElement { control_type, classname, name, automationid, attribute_count})
}


pub fn get_path_to_element<'a>(input: &mut &'a str) -> Result<Vec<XpathElement<'a>>> {
    let mut path_to_element: Vec<XpathElement<'a>> = Vec::new();
    // Skip the first element (the empty one) and the second element (the root element)
    let tree = input.split("/").skip(2).collect::<Vec<&str>>();
    
    printfmt!("Parsing XPath: {}", input);
    printfmt!("Tree: {:#?}", tree);

    for element in tree {
        // println!("\nParsing element: {}", element);       
        match parse_element(&mut element.as_ref()) {
            Ok(parsed_element) => path_to_element.push(parsed_element), // println!("Parsed element: {:?}", parsed_element),
            Err(e) => return Err(e), //,
        }
    }
    
    Ok(path_to_element)
}
// endregion: XPath parsing