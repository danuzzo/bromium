use time::{Duration, OffsetDateTime as DateTime};

use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

use eframe::egui;

use egui::Response;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows::Win32::Foundation::{POINT, RECT};



#[allow(unused)]
use crate::{rectangle, AppContext}; //winevent
use uitree::{get_all_elements_xml, SaveUIElementXML, UIElementInTreeXML, UITreeXML };

#[derive(Clone)]
struct TreeState {
    active_element: Option<SaveUIElementXML>,
    prev_element: Option<SaveUIElementXML>,
    clear_frame: bool,
    active_ui_element: Option<usize>,
    path_to_active_ui_element: Option<Vec<usize>>,
    refresh_path_to_active_ui_element: bool,
}

impl TreeState {
    fn new() -> Self {
        Self {
            active_element: None,
            prev_element: None,
            clear_frame: false,
            active_ui_element: None,
            path_to_active_ui_element: None,
            refresh_path_to_active_ui_element: false,
        }
    }

    fn update_state(&mut self, new_active_element: SaveUIElementXML, new_active_ui_element: usize) {

        // if we have an active element, check if the new one is different and if yes,
        // update the state to reflect the change
        if let Some(current_element) = &self.active_element {
            // only update the state if there is a change in the active element
            if new_active_element.get_element().get_runtime_id().unwrap() != current_element.get_element().get_runtime_id().unwrap() {
                self.prev_element = Some(current_element.clone());
                self.clear_frame = true;    
                self.active_element = Some(new_active_element);
                self.active_ui_element = Some(new_active_ui_element);
                self.refresh_path_to_active_ui_element = true;
            }
        } else {
            // there was no active element, so set the active element, 
            // and the active ui element to the proviced values and
            // default the prev element to the provided active
            self.prev_element = Some(new_active_element.clone());
            self.active_element = Some(new_active_element);
            self.active_ui_element = Some(new_active_ui_element);
            self.refresh_path_to_active_ui_element = true;
        }
    }

    fn update_path_to_active_ui_element(&mut self, ui_tree: &UITreeXML) {
        
        match self.active_ui_element {
            Some(active_ui_element) => {
                let path = ui_tree.get_tree().get_path_to_element(active_ui_element);
                self.path_to_active_ui_element = Some(path);
        
            },
            None => {
                self.path_to_active_ui_element  = None;
            }
        }
        self.refresh_path_to_active_ui_element = false;
    }

    // fn get_active_element_mut(&mut self) -> Option<&mut SaveUIElementXML> {
    //     if self.active_element.is_some() {
    //         self.active_element.as_mut()
    //     } else {
    //         None
    //     }        
    // }

}
#[derive(Clone)]
struct AppStatusMsg {
    status_msg: String,
    expiry: Option<DateTime>,
}

impl AppStatusMsg {
    #[allow(dead_code)]
    fn new(msg: String) -> Self {
        AppStatusMsg {
            status_msg: msg, 
            expiry: None,
        }
    }

    fn new_with_duration(msg: String, display_for_time: Duration) -> Self {
        
        let dur = display_for_time;
        let expiry = DateTime::now_utc() + dur;


        AppStatusMsg {
            status_msg: msg, 
            expiry: Some(expiry),
        }
    }

    fn has_display_duration(&self) -> bool {
        if let Some(_exp) = self.expiry {
            return true;
        }
        false    
    }

    fn is_expired(&self) -> bool {
        let now = DateTime::now_utc();
        if let Some(exp) = self.expiry {
            if now > exp {
                return true;
            }
        }
        false    
    }

}


struct HistoryEntry {
    summary: String,
    entries: Vec<String>,
}

#[derive(Default)]
struct DeduplicatedHistory {
    history: std::collections::VecDeque<HistoryEntry>,
}

impl DeduplicatedHistory {
    fn add(&mut self, summary: String, full: String) {
        if let Some(entry) = self.history.back_mut() {
            if entry.summary == summary {
                entry.entries.push(full);
                return;
            }
        }
        self.history.push_back(HistoryEntry {
            summary,
            entries: vec![full],
        });
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    fn ui(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                for HistoryEntry { summary, entries } in self.history.iter().rev() {
                    ui.horizontal(|ui| {
                        let response = ui.code(summary);
                        if entries.len() < 2 {
                            response
                        } else {
                            response | ui.weak(format!(" x{}", entries.len()))
                        }
                    })
                    .inner
                    .on_hover_ui(|ui| {
                        ui.spacing_mut().item_spacing.y = 4.0;
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        for entry in entries.iter().rev() {
                            ui.code(entry);
                        }
                    });
                }
            });
    }

}



// #[allow(dead_code)]
pub struct UIExplorer {
    app_context: AppContext,
    recording: bool,
    show_history: bool,
    highlighting: bool,
    ui_tree: UITreeXML,
    tree_state: Option<TreeState>,
    history: DeduplicatedHistory,
    status_msg: Option<AppStatusMsg>
}

impl UIExplorer {
    #[allow(dead_code)]
    pub fn new() -> Self {

        // get the ui tree in a separate thread
        let (tx, rx): (Sender<_>, Receiver<UITreeXML>) = channel();
        thread::spawn(|| {
            get_all_elements_xml(tx, None);
        });

        let ui_tree = rx.recv().unwrap();
        let app_context = AppContext::new_from_screen(0.4, 0.8);

        // TODO: Add winevent hook and ui automation instance

        Self {
            app_context,
            recording: false,
            show_history: false,
            highlighting: false,
            ui_tree,
            tree_state: None,
            history: DeduplicatedHistory::default(),
            status_msg: None,
        }


    }

    pub fn new_with_state(app_context: AppContext, ui_tree: UITreeXML) -> Self {

        // TODO: Add winevent hook and ui automation instance

        Self {
            app_context,
            recording: false,
            show_history: false,
            highlighting: false,
            ui_tree,
            tree_state: None,
            history: DeduplicatedHistory::default(),
            status_msg: None,
        }
    }


    fn render_ui_tree(&mut self, ui: &mut egui::Ui, state: &mut TreeState) {
        let tree = &self.ui_tree;
        Self::render_ui_tree_recursive(ui, tree, 0, state);
    }

    fn render_ui_tree_recursive(ui: &mut egui::Ui, tree: &UITreeXML, idx: usize, state: &mut TreeState) {
        
        for &child_index in tree.children(idx) {
            let (name, ui_element) = tree.node(child_index);

            // flag if this is the active element
            let mut is_active_element: bool = false;
            if let Some(active_id) = state.active_ui_element {
                if active_id == child_index {
                    is_active_element = true;
                }
            }
            
            if tree.children(child_index).is_empty() {
                // Node has no children, so just show a label
                let lbl = egui::Label::new(format!("  {}", name));
                let entry: Response;
                // let entry = ui.label(format!("  {}", name)).on_hover_cursor(egui::CursorIcon::Default);
                if is_active_element{
                    // show background to visually highlight the active element
                    let weak_bg_fill = ui.ctx().theme().default_visuals().widgets.inactive.weak_bg_fill;        
                    let tmp_entry = egui::Frame::new()
                    .fill(weak_bg_fill) 
                    .show(ui, |ui| {
                       ui.add(lbl).on_hover_cursor(egui::CursorIcon::Default);
                    });
                    entry = tmp_entry.response;
                } else {
                    // render standard label without any visual highlights
                    entry = ui.add(lbl).on_hover_cursor(egui::CursorIcon::Default);                    
                }
                
                if entry.clicked() {
                    state.update_state(ui_element.clone(), child_index);
                }
                if entry.hovered() {
                    entry.highlight();                    
                }
            }
            else {
                // Render children under collapsing header
                let header: egui::CollapsingHeader;                

                // println!("Header: {:?} - checking current element: {:?} against path {:?}", name, child_index, state.path_to_active_ui_element);
                if !is_in_path_to_active_element(child_index, &state.path_to_active_ui_element) {
                    // header is not on path, render a standard CollapsingHeader
                    // unless it's the root node (index = 1), in which case we want to show it open by default
                    if child_index == 1 {
                        header = egui::CollapsingHeader::new(name)
                            .id_salt(format!("ch_node{}", child_index))
                            .open(Some(true));
                    } else {
                        header = egui::CollapsingHeader::new(name)
                        .open(Some(false))
                        .id_salt(format!("ch_node{}", child_index))
                        }
                } else {
                    // println!("element is in path");
                    // println!("Element: {:?} ; {:?} is in path ; {:?}", name, child_index, state.path_to_active_ui_element);
                    if is_active_element {
                        // show background to visually highlight the active element
                        header = egui::CollapsingHeader::new(name)
                        .id_salt(format!("ch_node{}", child_index))
                        .open(Some(true))
                        .show_background(true);
                    } else {
                        header = egui::CollapsingHeader::new(name)
                        .id_salt(format!("ch_node{}", child_index))
                        .open(Some(true));

                    }
                }
                
                let header_resp = header
                    .show(ui, |ui| {
                        // Recursively render children
                        Self::render_ui_tree_recursive(ui, tree, child_index, state);
                    });    
                    
                if header_resp.header_response.clicked() {
                    state.update_state(ui_element.clone(), child_index);
                }
            }
        }
    }    

    fn process_event(&mut self, event: &egui::Event, state: &mut TreeState) {

        match event {
            egui::Event::MouseMoved { .. } => { 
                let cursor_position = unsafe {
                    let mut cursor_pos = POINT::default();
                    GetCursorPos(&mut cursor_pos).unwrap();
                    cursor_pos.x = (cursor_pos.x as f32 / self.app_context.screen_scale) as i32;
                    cursor_pos.y = (cursor_pos.y as f32 / self.app_context.screen_scale) as i32;
                    cursor_pos
                };
                                
                if let Some(ui_element_props) = rectangle::get_point_bounding_rect(&cursor_position, self.ui_tree.get_elements()) {
                    state.update_state(ui_element_props.get_element_props().clone(), ui_element_props.get_tree_index());
                } 
            }
            _ => (),
        }
    }


    fn set_status(&mut self, msg: String, duration: Duration) {
        let status_msg = AppStatusMsg::new_with_duration(msg, duration);
        self.status_msg = Some(status_msg);
    }

    fn clear_status(&mut self) {
        self.status_msg = None;
    }

}

impl eframe::App for UIExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // manage the TreeState
        let mut state: TreeState;
        if let Some(tree_state) = &self.tree_state { //.active_element 
            state = tree_state.clone();
        } else {
            state = TreeState::new();
        }        

        if state.refresh_path_to_active_ui_element {
            state.update_path_to_active_ui_element(&self.ui_tree);
            // println!("Path to active ui element {:?} set to : {:?}", state.active_ui_element,  state.path_to_active_ui_element);
        }

        // manage the AppStatusMsg lifecycle
        if let Some(status_msg) = &self.status_msg {
            if status_msg.is_expired() {
                self.clear_status();
            } else {
                if status_msg.has_display_duration() {
                    // switch from reactive mode to continuous mode to 
                    // ensure the status messages is cleared after the 
                    // specified time, even if there is no event triggered
                    ctx.request_repaint();
                }
            }
        }

        // status bar
        egui::TopBottomPanel::bottom("bottom_panel").resizable(false).show(ctx, |ui| {

            ui.add_space(2.0);
        
            ui.horizontal(|ui| {
                if let Some(msg) = &self.status_msg {
                    ui.label(&msg.status_msg);
                } else {
                    ui.label("Ready");
                }
                ui.label(format!("Clear Frame: {}", state.clear_frame));
            });
        
            ui.add_space(2.0);
        
        });
        

        // UI tree 
        egui::SidePanel::left("left_panel")
        .min_width(800.0)
        .max_width(1400.0)                
        .show(ctx, |ui| { // .min_width(300.0).max_width(600.0)

            egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                // printfmt!("running 'render_ui_tree' function on UIExplorer");
                self.render_ui_tree(ui, &mut state);

            });

        });

        // options bar
        egui::TopBottomPanel::top("top_panel").resizable(true).show(ctx, |ui| {

            ui.add_space(2.0);

            ui.input(|i| {
                
                for event in &i.raw.events {
    
                    if !self.recording && matches!(
                        event,
                        egui::Event::PointerMoved { .. }
                            | egui::Event::MouseMoved { .. }
                            | egui::Event::Touch { .. }
                    )
                {
                    continue;
                }
                    
                    // for the visual event summary
                    if self.show_history {
                        let summary = event_summary(event, self.ui_tree.get_elements());
                        let full = format!("{event:#?}");
                        self.history.add(summary, full);    
                    }

                    // update the actual active element
                    self.process_event(event, &mut state);
                }
            });
    
            ui.horizontal(|ui| {
                
                let prev_highlight = self.highlighting;
                ui.button("🔄").on_hover_text("Refresh");
                ui.add_space(2.0);
                ui.label(" | ");
                ui.add_space(2.0);
                ui.checkbox(&mut self.highlighting, "Show Highlight Rectangle");
                ui.checkbox(&mut self.recording, "Track Cursor");
                if self.recording {
                    ui.checkbox(&mut self.show_history, "Show Event History");
                }
                let new_highlight = self.highlighting;
                
                // clear any highlighted surrounding rectangle as 
                if new_highlight != prev_highlight && new_highlight == false {
                    printfmt!("Old highlight value was {}, new one is {}", prev_highlight, new_highlight);
                    let rect: RECT = RECT { 
                        left: 0, 
                        top: 0, 
                        right: self.app_context.screen_width, 
                        bottom: self.app_context.screen_height, 
                    };
                    rectangle::clear_frame(rect).unwrap();
                    state.clear_frame = false;
                }
                
            });

            ui.add_space(2.0);

            if self.show_history {
                ui.add_space(6.0);
                self.history.ui(ui);
            }
            

        });

        
        // main screen with element details
        egui::CentralPanel::default().show(ctx, |ui| {
                
            ui.horizontal(|ui| {

                if let Some(active_element) = &state.active_element {
                    
                    // Optionally render the frame around the active element on the screen
                    if self.highlighting {
                        let left: f32 = active_element.get_element().get_bounding_rectangle().unwrap().get_left() as f32 * self.app_context.screen_scale;
                        let top: f32 = active_element.get_element().get_bounding_rectangle().unwrap().get_top() as f32 * self.app_context.screen_scale;
                        let right: f32 = active_element.get_element().get_bounding_rectangle().unwrap().get_right() as f32 * self.app_context.screen_scale;
                        let bottom: f32 = active_element.get_element().get_bounding_rectangle().unwrap().get_bottom() as f32 * self.app_context.screen_scale;

                        let rect: RECT = RECT { 
                            left: left as i32, 
                            top: top as i32, 
                            right: right as i32, 
                            bottom: bottom as i32, 
                        };
                        
                        if let Some(prev_element) = &state.prev_element {
                            let prev_left: f32 = prev_element.get_element().get_bounding_rectangle().unwrap().get_left() as f32 * self.app_context.screen_scale;
                            let prev_top: f32 = prev_element.get_element().get_bounding_rectangle().unwrap().get_top() as f32 * self.app_context.screen_scale;
                            let prev_right: f32 = prev_element.get_element().get_bounding_rectangle().unwrap().get_right() as f32 * self.app_context.screen_scale;
                            let prev_bottom: f32 = prev_element.get_element().get_bounding_rectangle().unwrap().get_bottom() as f32 * self.app_context.screen_scale;

                            let prev_rect: RECT = RECT {
                                left: prev_left as i32, 
                                top: prev_top as i32, 
                                right: prev_right as i32, 
                                bottom: prev_bottom as i32,     
                            };
                            if state.clear_frame { //rect != prev_rect && 
                                printfmt!("Cleanup needed - new: {:?} vs old: {:?}", rect, prev_rect);
                                rectangle::clear_frame(prev_rect).unwrap();
                                rectangle::draw_frame(rect, 4).unwrap();
                                state.clear_frame = false;
                            } else {
                                rectangle::draw_frame(rect, 4).unwrap();
                            }
                        } else {
                            rectangle::draw_frame(rect, 4).unwrap();
                        }
                    } 
                    
                    // display the element properties 
                    egui::Grid::new("some_unique_id").min_col_width(100.0).max_col_width(800.0)
                    .show(ui, |ui| {
                        ui.label("Name:");
                        ui.label(active_element.get_element().get_name().unwrap_or_default());
                        ui.end_row();
                    
                        ui.label("Control Type:");
                        let mut control_type: String = "".to_string();
                        if let Ok(ctrl_type) =  active_element.get_element().get_control_type() {
                            control_type = ctrl_type.to_string();    
                        }
                        ui.label(control_type);
                        ui.end_row();

                        ui.label("Localized Control Type:");
                        ui.label(active_element.get_element().get_localized_control_type().unwrap_or_default());
                        if ui.button("📋").clicked() {
                            ui.ctx().copy_text(active_element.get_element().get_localized_control_type().unwrap_or_default());
                            self.set_status("Value copied to clipboard".to_string(), Duration::seconds(2));
                        }
                        ui.end_row();

                        ui.label("Framework ID:");
                        ui.label(active_element.get_element().get_framework_id().unwrap_or_default());
                        ui.end_row();

                        ui.label("Class Name:");
                        ui.label(active_element.get_element().get_classname().unwrap_or_default());
                        if ui.button("📋").clicked() {
                            ui.ctx().copy_text(active_element.get_element().get_classname().unwrap_or_default());
                            self.set_status("Value copied to clipboard".to_string(), Duration::seconds(2));
                        }
                        ui.end_row();

                        ui.label("Runtime ID:");
                        ui.label(active_element.get_element().get_runtime_id().unwrap_or(Vec::new()).iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-"));
                        ui.end_row();

                        ui.label("Surrounding Rectangle:");
                        ui.label(format!("{:?}", active_element.get_element().get_bounding_rectangle().unwrap_or(uiautomation::types::Rect::new(0, 0, 0, 0))));
                        ui.end_row();
                        
                        ui.label("level:");
                        ui.label(active_element.level.to_string());
                        ui.end_row();
                        
                        ui.label("z-order:");
                        ui.label(active_element.z_order.to_string());
                        ui.end_row();

                        ui.label("Automation ID:");
                        ui.label(active_element.get_element().get_automation_id().unwrap_or_default()); 
                        ui.end_row();


                        let xpath = self.ui_tree.get_xpath_for_element(state.active_ui_element.unwrap_or(0));
                        ui.label("XPath:");
                        ui.label(xpath.clone());
                        if ui.button("📋").clicked() {
                            ui.ctx().copy_text(xpath);
                            self.set_status("XPath copied to clipboard".to_string(), Duration::seconds(2));
                        }
                    });    

                }
                else {
                    ui.label("No active element");
                }

            });

    
        });



        // self.active_element = state.active_element;
        self.tree_state = Some(state);
    }



}


fn event_summary(event: &egui::Event, ui_elements: &Vec<UIElementInTreeXML>) -> String {
    match event {
        egui::Event::PointerMoved { .. }   => {        
            "PointerMoved { .. }".to_owned()
        }
        egui::Event::MouseMoved { .. } => { 
            let cursor_position = unsafe {
                let mut cursor_pos = POINT::default();
                GetCursorPos(&mut cursor_pos).unwrap();
                cursor_pos
            };

            if let Some(ui_element_props) = rectangle::get_point_bounding_rect(&cursor_position, ui_elements) {
                // format!("MouseMoved {{ x: {}, y: {} }} over {}", cursor_position.x, cursor_position.y, ui_element_props.name)
                let ui_element_props = ui_element_props.get_element_props();
                let mut control_type: String = "".to_string();
                if let Ok(ctrl_type) =  ui_element_props.get_element().get_control_type() {
                    control_type = ctrl_type.to_string();    
                }
        
                format!("MouseMoved over {{ name: '{}', control_type: '{}' bounding_rect: {} }}", ui_element_props.get_element().get_name().unwrap_or_default(), control_type, ui_element_props.get_element().get_bounding_rectangle().unwrap_or(uiautomation::types::Rect::new(0, 0, 0, 0)))
            } else {
            // format!("MouseMoved {{ x: {}, y: {} }} ", cursor_position.x, cursor_position.y)
            "MouseMoved { .. }".to_owned()
            }
        }
        egui::Event::Zoom { .. } => "Zoom { .. }".to_owned(),
        egui::Event::Touch { phase, .. } => format!("Touch {{ phase: {phase:?}, .. }}"),
        egui::Event::MouseWheel { unit, .. } => format!("MouseWheel {{ unit: {unit:?}, .. }}"),

        _ => format!("{event:?}"),
    }
}

fn is_in_path_to_active_element(current_element: usize, path_to_active_element: &Option<Vec<usize>>) -> bool {
    
    if path_to_active_element.is_none() {
        return false;
    }

    let path_to_active_element = path_to_active_element.as_ref().unwrap();
    for &index in path_to_active_element {
        if index == current_element {
            return true;
        }
    }
    false
}