use skia_safe::{Canvas, Color, Paint, PathBuilder, Point};

pub fn draw_music_icon(canvas: &Canvas, cx: f32, cy: f32, alpha: u8) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(alpha, 255, 255, 255));
    paint.set_style(skia_safe::paint::Style::Fill);

    let head_r = 3.5;
    let left_head = Point::new(cx - 5.0, cy + 6.0);
    let right_head = Point::new(cx + 5.0, cy + 3.0);
    
    canvas.draw_circle(left_head, head_r, &paint);
    canvas.draw_circle(right_head, head_r, &paint);

    let mut builder = PathBuilder::new();
    
    let stem_width = 1.8;
    
    builder.move_to(Point::new(cx - 5.0 + head_r - stem_width, cy - 9.0)); 
    builder.line_to(Point::new(cx + 5.0 + head_r - stem_width, cy - 12.0)); 
    builder.line_to(Point::new(cx + 5.0 + head_r, cy - 12.0)); 
    builder.line_to(Point::new(cx + 5.0 + head_r, cy + 3.0)); 
    builder.line_to(Point::new(cx + 5.0 + head_r - stem_width, cy + 3.0)); 
    builder.line_to(Point::new(cx + 5.0 + head_r - stem_width, cy - 9.5)); 
    builder.line_to(Point::new(cx - 5.0 + head_r - stem_width, cy - 6.5)); 
    builder.line_to(Point::new(cx - 5.0 + head_r - stem_width, cy + 6.0)); 
    builder.line_to(Point::new(cx - 5.0 + head_r - 2.0 * stem_width, cy + 6.0)); 
    builder.line_to(Point::new(cx - 5.0 + head_r - 2.0 * stem_width, cy - 9.0)); 
    builder.close();

    let path = builder.detach();
    canvas.draw_path(&path, &paint);

    let mut beam_builder = PathBuilder::new();
    let beam_h = 4.0;
    beam_builder.move_to(Point::new(cx - 5.0 + head_r - 2.0 * stem_width, cy - 9.0));
    beam_builder.line_to(Point::new(cx + 5.0 + head_r, cy - 12.0));
    beam_builder.line_to(Point::new(cx + 5.0 + head_r, cy - 12.0 + beam_h));
    beam_builder.line_to(Point::new(cx - 5.0 + head_r - 2.0 * stem_width, cy - 9.0 + beam_h));
    beam_builder.close();
    
    canvas.draw_path(&beam_builder.detach(), &paint);
}
