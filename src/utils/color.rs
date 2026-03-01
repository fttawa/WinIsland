use skia_safe::Color;

pub const COLOR_BG: Color = Color::from_rgb(28, 28, 30);
pub const COLOR_CARD: Color = Color::from_rgb(44, 44, 46);
pub const COLOR_CARD_HIGHLIGHT: Color = Color::from_rgb(63, 63, 66);
pub const COLOR_ACCENT: Color = Color::from_rgb(10, 132, 255);
pub const COLOR_TEXT_PRI: Color = Color::WHITE;
pub const COLOR_TEXT_SEC: Color = Color::from_rgb(142, 142, 147);
pub const COLOR_DANGER: Color = Color::from_rgb(255, 69, 58);
pub const COLOR_DISABLED: Color = Color::from_rgb(60, 60, 60);

pub fn get_island_border_weights(cx: i32, cy: i32, w: f32, h: f32) -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
}
