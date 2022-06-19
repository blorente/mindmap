pub struct RenderItemSettings {
    pub margin: f32,
    pub font: String,
    pub border_radius: f32,
    pub border_weight: f32,
}
impl RenderItemSettings {
    pub fn new() -> Self {
        return RenderItemSettings {
            margin: 10.0,
            font: "".to_string(),
            border_radius: 20.0,
            border_weight: 10.0,
        };
    }
}
