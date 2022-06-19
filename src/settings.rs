use femtovg::{Color, FontId};

pub struct RenderItemSettings {
    pub margin: f32,
    pub font: Option<FontId>,
    pub font_size: f32,
    pub border_radius: f32,
    pub border_weight: f32,
    pub text_color: Color,
}
impl RenderItemSettings {
    pub fn new() -> Self {
        return RenderItemSettings {
            margin: 10.0,
            font_size: 18.0,
            border_radius: 20.0,
            border_weight: 10.0,
            font: None,
            text_color: Color::rgb(20, 100, 255),
        };
    }
}

pub struct Settings {
    pub render_item_settings: RenderItemSettings,
    pub font_path: String,
}
impl Settings {
    pub fn new() -> Self {
        Settings {
            render_item_settings: RenderItemSettings::new(),
            font_path: "/System/Library/Fonts/Courier.ttc".to_string(),
        }
    }
}
