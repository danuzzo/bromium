use uiautomation::controls::ControlType;

pub trait ConvertFromControlType {
    fn as_str<'a>(&self) -> &'a str;
}

impl ConvertFromControlType for ControlType {
    fn as_str<'a>(&self) -> &'a str {
        match &self {
            ControlType::Button => "Button",
            ControlType::Calendar => "Calendar",
            ControlType::CheckBox => "CheckBox",
            ControlType::ComboBox => "ComboBox",
            ControlType::Edit => "Edit",
            ControlType::Hyperlink => "Hyperlink",
            ControlType::Image => "Image",
            ControlType::ListItem => "ListItem",
            ControlType::List => "List",
            ControlType::Menu => "Menu",
            ControlType::MenuBar => "MenuBar",
            ControlType::MenuItem => "MenuItem",
            ControlType::ProgressBar => "ProgressBar",
            ControlType::RadioButton => "RadioButton",
            ControlType::ScrollBar => "ScrollBar",
            ControlType::Slider => "Slider",
            ControlType::Spinner => "Spinner",
            ControlType::StatusBar => "StatusBar",
            ControlType::Tab => "Tab",
            ControlType::TabItem => "TabItem",
            ControlType::Text => "Text",
            ControlType::ToolBar => "ToolBar",
            ControlType::ToolTip => "ToolTip",
            ControlType::Tree => "Tree",
            ControlType::TreeItem => "TreeItem",
            ControlType::Custom => "Custom",
            ControlType::Group => "Group",
            ControlType::Thumb => "Thumb",
            ControlType::DataGrid => "DataGrid",
            ControlType::DataItem => "DataItem",
            ControlType::Document => "Document",
            ControlType::SplitButton => "SplitButton",
            ControlType::Window => "Window",
            ControlType::Pane => "Pane",
            ControlType::Header => "Header",
            ControlType::HeaderItem => "HeaderItem",
            ControlType::Table => "Table",
            ControlType::TitleBar => "TitleBar",
            ControlType::Separator => "Separator",
            ControlType::SemanticZoom => "SemanticZoom",
            ControlType::AppBar => "AppBar",
        }
    }
}