use crate::conversion::ConvertFromControlType;

use crate::save_ui_element::SaveUIElement;

// use crate::commons::FileWriter;
use crate::{printfmt, UITreeMap};
use xmlutil::xml::{XMLDomWriter, XMLDomNode};
use xmlutil::xpath_gen::get_xpath_full_from_runtime_id; //get_xpath_from_runtime_id, 
use xmlutil::xpath_eval::eval_xpath;
use xmlutil::XpathQueryResult;



use std::sync::mpsc::Sender;

use uiautomation::core::UIAutomation;
use uiautomation::{UIElement, UITreeWalker};


#[derive(Debug)]
pub struct UIElementInTree {
    element_props: SaveUIElement,
    tree_index: usize,
}

impl UIElementInTree {
    pub fn new(element_props: SaveUIElement, tree_index: usize) -> Self {
        UIElementInTree {element_props, tree_index}
    }

    pub fn get_element_props(&self) -> &SaveUIElement {
        &self.element_props
    }

    pub fn get_tree_index(&self) -> usize {
        self.tree_index
    }
}

#[derive(Debug)]
pub struct UITree {
    tree: UITreeMap<SaveUIElement>,
    xml_dom_tree: String,
    ui_elements: Vec<UIElementInTree>,
}

impl UITree {
    pub fn new(tree: UITreeMap<SaveUIElement>, xml_dom_tree: String, ui_elements: Vec<UIElementInTree>) -> Self {
        UITree {tree, xml_dom_tree, ui_elements} 
    }

    pub fn get_tree(&self) -> &UITreeMap<SaveUIElement> {
        &self.tree
    }

    pub fn get_xml_dom_tree(&self) -> &str {
        &self.xml_dom_tree
    }

    pub fn get_elements(&self) -> &Vec<UIElementInTree> {
        &self.ui_elements
    }

    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut(usize, &SaveUIElement),
    {
        self.tree.for_each(f);
    }

    pub fn root(&self) -> usize {
        self.tree.root()
    }

    pub fn children(&self, index: usize) -> &[usize] {
        self.tree.children(index)
    }

    pub fn node(&self, index: usize) -> (&str, &SaveUIElement) {
        let node = &self.tree.node(index);
        (&node.name, &node.data)

    }
    
    pub fn get_xpath_for_element(&self, index: usize) -> String {

        let node = &self.tree.node(index);
        let save_ui_elem = &node.data;
        // let rt_id = save_ui_elem.get_element().get_runtime_id().unwrap_or(vec![0, 0, 0, 0]).iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
        let rt_id = save_ui_elem.get_element().get_runtime_id().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
        get_xpath_full_from_runtime_id(rt_id.as_str(), self.get_xml_dom_tree())

    }

    pub fn get_element_by_xpath(&self, xpath: &str) -> Option<&SaveUIElement> {

        // Patch the xpath with /@RtID if it is missing
        let xpath = if !xpath.ends_with("/@RtID") {xpath.to_string() + "/@RtID"} else {xpath.to_string()};

        let xpath_result = eval_xpath(xpath, self.get_xml_dom_tree().to_string());
        
        match xpath_result.get_result_count() {
            0 => return None,
            1 => {
                let items = xpath_result.get_result_items();
                let default_result = &XpathQueryResult::default();
                let itm = items.get(0).unwrap_or(default_result);
                let runtime_id = itm.get_item_value();
                let ui_elem = self.get_tree().get_element_by_runtime_id(runtime_id).unwrap();
                let ui_elem = ui_elem.data.get_element();
                return Some(ui_elem);
            },
            _ => {
                printfmt!("Warning: XPath expression returned {} results, expected only 1 result. Returning the first result.", xpath_result.get_result_count());
                return None;
            }
        }


    }

}


// #[derive(Debug, Clone)]
// pub struct SaveUIElement {
//     pub element: UIElement,
//     pub bounding_rect_size: i32,
//     pub level: usize,
//     pub z_order: usize,
//     pub xpath: Option<String>,

// }

// unsafe impl Send for SaveUIElement {}
// unsafe impl Sync for SaveUIElement {}

// impl SaveUIElement {
//     pub fn new(element: UIElement, level: usize, z_order: usize) -> Self {
//         let bounding_rect: uiautomation::types::Rect = element.get_bounding_rectangle().unwrap_or(uiautomation::types::Rect::new(0, 0, 0, 0));
//         let bounding_rect_size: i32 = (bounding_rect.get_right() - bounding_rect.get_left()) * (bounding_rect.get_bottom() - bounding_rect.get_top());            
//         SaveUIElement { element, bounding_rect_size, level, z_order, xpath: None}
//     }

//     pub fn get_element(&self) -> &UIElement {
//         &self.element
//     }

//     pub fn set_xpath(&mut self, xpath: String) {
//         self.xpath = Some(xpath)
//     }
// }


pub fn get_all_elements_xml(tx: Sender<UITree>, max_depth: Option<usize>, calling_window_caption: Option<String>) {   
    
    let automation = UIAutomation::new().unwrap();
    // control view walker
    let walker = automation.get_control_view_walker().unwrap();

    // allocate a new ui elements vector with a capacity of 10000 elements
    let mut ui_elements: Vec<UIElementInTree> = Vec::with_capacity(10000);

    let mut xml_writer = XMLDomWriter::new();

    // get the desktop and all UI elements below the desktop
    let root = automation.get_root_element().unwrap();
    let runtime_id = root.get_runtime_id().unwrap_or(vec![0, 0, 0, 0]).iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
    let item = format!("'{}' {} ({} | {} | {})", root.get_name().unwrap(), root.get_localized_control_type().unwrap(), root.get_classname().unwrap(), root.get_framework_id().unwrap(), runtime_id);
    let ui_elem_props = SaveUIElement::new(root.clone(), 0, 999);
    let mut tree = UITreeMap::new(item, runtime_id.clone(), ui_elem_props.clone());
    let ui_elem_in_tree = UIElementInTree::new(ui_elem_props, 0);    
    // let mut ui_elements: Vec<UIElementInTree> = vec![ui_elem_in_tree];
    ui_elements.push(ui_elem_in_tree);
    xml_writer.set_root(XMLDomNode::new(root.get_classname().unwrap().as_str()));
    let xml_root = xml_writer.get_root_mut().unwrap();
    xml_root.set_attribute("RtID", runtime_id.as_str());
    xml_root.set_attribute("Name", root.get_name().unwrap_or("No name defined".to_string()).as_str());

    if let Ok(_first_child) = walker.get_first_child(&root) {     
        // itarate over all child ui elements
        get_element(&mut tree, &mut ui_elements,  0, &walker, &root, xml_root, 0, 0, max_depth, calling_window_caption);
    }

    // creating the XML DOM tree
    let xml_dom_tree = xml_writer.to_string().unwrap();

    // sorting the elements by z_order and then by ascending size of the bounding rectangle
    printfmt!("Sorting UI elements by size and z-order...");
    ui_elements.sort_by(|a, b| a.get_element_props().get_bounding_rect_size().cmp(&b.get_element_props().get_bounding_rect_size()));
    ui_elements.sort_by(|a, b| a.get_element_props().get_z_order().cmp(&b.get_element_props().get_z_order()));

    // DEBUG ONLY
    // let mut fw = FileWriter::new("uiexplorer_xml");
    // fw.write(&xml_dom_tree.as_str());

    // pack the tree and ui_elements vector into a single struct
    let ui_tree = UITree::new(tree, xml_dom_tree, ui_elements);

    // send the tree containing all UI elements back to the main thread
    printfmt!("Sending UI tree with {} elements to the main thread...", ui_tree.get_elements().len());
    match tx.send(ui_tree) {
        Ok(_) => {printfmt!("UI tree sent successfully.");}
        Err(e) => {printfmt!("Error sending UI tree: {:?}", e);}
    };

}


fn get_element(mut tree: &mut UITreeMap<SaveUIElement>, mut ui_elements: &mut Vec<UIElementInTree>, parent: usize, walker: &UITreeWalker, element: &UIElement, xml_dom_node: &mut XMLDomNode, level: usize, mut z_order: usize, max_depth: Option<usize>, calling_window_caption: Option<String>)  {
    if let Some(limit) = max_depth {
        if level > limit {
            return;
        }    
    }

    if let Some(caption) = &calling_window_caption {
        if let Ok(name) = element.get_name() {
            if name == *caption {
                // printfmt!("Skipping element with caption: {}", caption);
                return;
            }
        }
    }

    let runtime_id = element.get_runtime_id().unwrap_or(vec![0, 0, 0, 0]).iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
    let item = format!("'{}' {} ({} | {} | {})", element.get_name().unwrap_or_default(), element.get_localized_control_type().unwrap_or_default(), element.get_classname().unwrap_or_default(), element.get_framework_id().unwrap_or_default(), runtime_id);
    let ui_elem_props: SaveUIElement;

    if level == 0 {
        // manually setting the z_order for the root element
        ui_elem_props = SaveUIElement::new(element.clone(), level, 999);
    } else {
        ui_elem_props = SaveUIElement::new(element.clone(), level, z_order);
    }
    
    let parent = tree.add_child(parent, item.as_str(), &runtime_id.as_str(), ui_elem_props.clone());
    let ui_elem_in_tree = UIElementInTree::new(ui_elem_props, parent);
    ui_elements.push(ui_elem_in_tree);
        
    let curr_xml_dom_node = xml_dom_node.add_child(XMLDomNode::new(element.get_control_type().unwrap().as_str()));
    curr_xml_dom_node.set_attribute("RtID", runtime_id.as_str());
    curr_xml_dom_node.set_attribute("Name", element.get_name().unwrap_or("No name defined".to_string()).as_str());
    

    // Walking the children of the current element
    if let Ok(child) = walker.get_first_child(&element) {
        // getting child elements
        // printfmt!("Found child element: {}", child.get_name().unwrap_or("Unknown".to_string()));
        get_element(&mut tree, &mut ui_elements, parent, walker, &child, curr_xml_dom_node, level + 1, z_order, max_depth, calling_window_caption.clone());
        let mut next = child;
        // walking siblings
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            // incrementing z_order for each sibling
            if level + 1 == 1 {
                z_order += 1;
            }
            // printfmt!("Found sibling element: {}", sibling.get_name().unwrap_or("Unknown".to_string()));
            get_element(&mut tree, &mut ui_elements, parent, walker, &sibling, curr_xml_dom_node,  level + 1, z_order, max_depth, calling_window_caption.clone());
            next = sibling;
        }
    }    
    
}

