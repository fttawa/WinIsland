use skia_safe::{Canvas, Color, Paint, Rect};
pub fn draw_settings_icon(canvas: &Canvas, cx: f32, cy: f32, alpha: u8, scale: f32) {
    let mut paint = Paint::default();
    paint.set_color(Color::from_argb(alpha, 220, 220, 220));
    paint.set_anti_alias(true);
    paint.set_style(skia_safe::paint::Style::Fill);
    canvas.save();
    canvas.translate((cx, cy));
    canvas.draw_circle((0.0, 0.0), 6.5 * scale, &paint);
    for i in 0..8 {
        canvas.save();
        canvas.rotate(i as f32 * 45.0, None);
        let tooth = Rect::from_xywh(-2.0 * scale, -9.0 * scale, 4.0 * scale, 4.0 * scale);
        canvas.draw_round_rect(tooth, 1.5 * scale, 1.5 * scale, &paint);
        canvas.restore();
    }
    paint.set_color(Color::from_argb(alpha, 0, 0, 0));
    canvas.draw_circle((0.0, 0.0), 3.0 * scale, &paint);
    canvas.restore();
}
