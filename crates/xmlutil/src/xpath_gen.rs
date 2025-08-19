
use roxmltree::{Document, Node};

/// Check if an attribute uniquely identifies the node among all nodes in the document.
fn is_attribute_unique(doc: &Document, node: Node, attr_name: &str) -> bool {
    if let Some(attr_value) = node.attribute(attr_name) {
        let count = doc
            .descendants()
            .filter(|n| n.attribute(attr_name) == Some(attr_value))
            .count();
        return count == 1;
    }
    false
}

/// Generate a robust, ROBULA+-like XPath for the given node.
fn get_xpath_robula(doc: &Document, node: Node) -> String {
    // Rule 1: Prefer globally unique attribute
    for attr in ["id", "name"] {
        if is_attribute_unique(doc, node, attr) {
            return format!("//*[@{}='{}']", attr, node.attribute(attr).unwrap());
        }
    }

    // Build full path up to the root with optimization rules
    let mut path_parts = Vec::new();
    let mut current = Some(node);

    while let Some(n) = current {
        if n.is_element() {
            let tag = n.tag_name().name();

            // Try using unique attribute in parent scope
            if is_attribute_unique(doc, n, "name") {
                path_parts.push(format!("{}[@name='{}']", tag, n.attribute("name").unwrap()));
                break;
            }

            // Determine if this node needs an index
            let parent = n.parent();
            let same_tag_count = parent.map_or(1, |p| {
                p.children()
                    .filter(|c| c.is_element() && c.tag_name().name() == tag)
                    .count()
            });

            if same_tag_count > 1 {
                // Count this node's position among siblings
                let mut index = 1;
                let mut prev = n.prev_sibling();
                while let Some(sib) = prev {
                    if sib.is_element() && sib.tag_name().name() == tag {
                        index += 1;
                    }
                    prev = sib.prev_sibling();
                }
                path_parts.push(format!("{}[{}]", tag, index));
            } else {
                path_parts.push(tag.to_string());
            }
        }
        current = n.parent();
    }

    path_parts.reverse();
    format!("/{}", path_parts.join("/"))
}



// pub fn get_xpath_from_runtime_id(runtime_id: String, xml: &str) -> String {

//     let root_node = RNode::new_document();
//     parse(root_node.clone(), xml, None).unwrap();

//     let target = root_node
//     .descend_iter()
//     .find(|n| n.attribute("RtID") == Some(runtime_id.clone()))
//     .unwrap();

//     get_xpath_from_rnode(&root_node, &target)

// }

pub fn get_xpath_full_from_runtime_id(runtime_id: &str, xml: &str) -> String {

    let doc = Document::parse(xml).unwrap();

    if let Some(node_id) = doc
        .descendants()
        .find(|n| n.attribute("RtID") == Some(runtime_id)) {
            get_xpath_robula(&doc, node_id)
        } else {
            "UI Element not found - no xpath available".to_string()
        }


    

}