use xrust::item::Node; 
use xrust::qname::QualifiedName;
// The Node trait and xml tree model
use xrust::trees::smite::RNode;
// use xrust::parser::xml::parse;
// use xrust::item::Item;
// use xrust::transform::context::Context;
use xrust::value::ValueData;

trait FetchAttribute {
    fn attribute(&self, name: &str) -> Option<String>;
}

impl FetchAttribute for RNode {
    fn attribute(&self, name: &str) -> Option<String> {
        let val = self.get_attribute(&QualifiedName::new(None, None, name));
        match &val.value {
            ValueData::String(s) => Some(s.to_owned()),
            _ => None,
        }
    }
}

/// Check if an attribute uniquely identifies the node within the document
fn is_attr_unique(root: &RNode, node: &RNode, attr: &str) -> bool {
    
    if let Some(val) = node.attribute(attr) {
        let mut count = 0;
        for descendant in root.descend_iter() {
            if let Some(v) = descendant.attribute(attr) {
                if v == val {
                    count += 1;
                    if count > 1 {
                        return false;
                    }
                }
            }
        }
        return count == 1;
    }
    false
}

/// Generate ROBULA+-style XPath using `xrust` tree navigation
pub fn get_xpath(root: &RNode, node: &RNode) -> String {
    // Prefer globally unique id or name
    for attr in &["id", "name"] {
        if is_attr_unique(root, node, attr) {
            let val = node.attribute(attr).unwrap();
            return format!("//*[@{}='{}']", attr, val);
        }
    }

    // Otherwise build path up to root
    let mut parts = Vec::new();
    let mut current_opt = Some(node.clone());

    while let Some(current) = current_opt {
        if current.is_element() {
            let tag = current.name().localname_to_string();

            // Check if `name` attribute is unique here
            // FIXME: This check with the break is leading to an inconsistent XPath
            //       generation, as it stops at the first unique name.
            if is_attr_unique(root, &current, "name") {
                let val = current.attribute("name").unwrap();
                parts.push(format!("{}[@name='{}']", tag, val));
                break;
            }

            // Determine if this node needs an index
            let parent = current.parent();
            let same_tag_count = parent.map_or(1, |p| {
                p.descend_iter()
                    .filter(|c| c.is_element() && c.name().localname_to_string() == tag)
                    .count()
            });

            if same_tag_count > 1 {
                // Count this node's position among siblings
                let mut index = 1;
                let mut prev = current.prev_iter().next();
                while let Some(sib) = prev {
                    if sib.is_element() && sib.name().localname_to_string() == tag {
                        index += 1;
                    }
                    prev = sib.prev_iter().next();
                }
                parts.push(format!("{}[{}]", tag, index));
            } else {
                parts.push(tag.to_string());
            }



        } // if current.is_element()
        current_opt = current.parent();
    }

    parts.reverse();
    format!("/{}", parts.join("/"))
}

pub fn get_xpath_full(node: &RNode) -> String {

    // Build path up to root
    let mut parts = Vec::new();
    let mut current_opt = Some(node.clone());

    while let Some(current) = current_opt {
        if current.is_element() {
            let tag = current.name().localname_to_string();

            // Determine if this node needs an index
            let parent = current.parent();
            let same_tag_count = parent.map_or(1, |p| {
                p.descend_iter()
                    .filter(|c| c.is_element() && c.name().localname_to_string() == tag)
                    .count()
            });

            if same_tag_count > 1 {
                // Count this node's position among siblings
                let mut index = 1;
                let mut prev = current.prev_iter().next();
                while let Some(sib) = prev {
                    if sib.is_element() && sib.name().localname_to_string() == tag {
                        index += 1;
                    }
                    prev = sib.prev_iter().next();
                }
                parts.push(format!("{}[{}]", tag, index));
            } else {
                parts.push(tag.to_string());
            }



        } // if current.is_element()
        current_opt = current.parent();
    }

    parts.reverse();
    format!("/{}", parts.join("/"))
}