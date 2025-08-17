#![allow(dead_code)]

use std::rc::Rc;

use xrust::ErrorKind;

use xrust::item::{Item, Node, NodeType}; //SequenceTrait
use xrust::parser::xml::parse as xmlparse;
use xrust::parser::xpath::parse;
use xrust::transform::context::{ContextBuilder, StaticContextBuilder};
use xrust::trees::smite::RNode;
use xrust::xdmerror::Error;


pub struct XpathResult {
    result_count: usize,
    result: Vec<Item<Rc<xrust::trees::smite::Node>>>,
}

impl XpathResult {
    fn new(result_count: usize, result: Vec<Item<Rc<xrust::trees::smite::Node>>>) -> Self {
        XpathResult { result_count, result }
    }
    
    fn get_result_count(&self) -> usize {
        self.result_count
    }
    
    fn get_result_items(&self) -> Vec<Item<Rc<xrust::trees::smite::Node>>> {
        self.result.clone() 
    }
}



pub fn eval_xpath(expr: String, srcxml: String) -> XpathResult {
    
    // Parse the XPath expression
    let xpath = parse::<RNode>(expr.trim(), None).expect("XPath expression not recognised");

    // Parse the XML into a RNode
    let root = RNode::new_document();
    xmlparse(root.clone(), srcxml.as_str(), None).expect("unable to parse XML");

    // Create a dynamic transformation context
    let context = ContextBuilder::new()
        .context(vec![Item::Node(root)])
        .build();
    // Create a static transformation contact
    // with dummy callbacks
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    // Evaluate the XPath expression
    // against the context.
    let result = context
        .dispatch(&mut stctxt, &xpath)
        .expect("failed to evaluate XPath");

    // Print the result
    if result.is_empty() {
        println!("No results found for XPath expression: {}", expr.trim());
    } else {
        println!("Results for XPath expression: {}", expr.trim());
        println!("Number of results: {}", result.len());
        
        for item in &result {
            match item {
                Item::Node(node) => {
                                        
                    // Print the node information
                    let node_type = node.node_type();
                    match node_type {
                        NodeType::Element => {
                            println!("Element name: {}, value: {}", node.name(), node.to_string());
                        }
                        NodeType::Text => {
                            println!("Text content for name ({}): {}", node.name(), node.value()  );
                        }
                        NodeType::Attribute => {
                            println!("Atttribute: {} = {}", node.name(), node.value());
                        }
                        _ => {
                            println!("Other node type: {:?}", node);
                        }
                    }
                    
                }
                Item::Value(v) => {
                    // Print the atomic value
                    println!("value: {}", v.to_string());
                }
                _ => {
                    println!("Unsupported item type: {:?}", item);
                }
            }
        }
    }

    XpathResult::new(result.len(), result)

}
