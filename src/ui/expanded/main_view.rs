use skia_safe::{
    Canvas, Paint, Color, Font, FontStyle, FontMgr, Rect, RRect,
    PathBuilder, Point, Data, Image, SamplingOptions, FilterMode, MipmapMode, Typeface
};
use crate::icons::arrows::draw_arrow_right;
use crate::core::smtc::MediaInfo;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static IMG_CACHE: RefCell<Option<(String, Image)>> = RefCell::new(None);
    static FONT_MGR: FontMgr = FontMgr::new();
    static FALLBACK_CACHE: RefCell<HashMap<(char, u32), Typeface>> = RefCell::new(HashMap::new());
    static TEXT_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

fn get_font_mgr() -> FontMgr {
    FONT_MGR.with(|mgr| mgr.clone())
}

fn style_to_key(style: FontStyle) -> u32 {
    let weight = *style.weight() as u32; 
    let width = *style.width() as u32;  
    let slant = style.slant() as u32;    
    (weight << 16) | (width << 8) | slant
}

fn get_typeface_for_char(c: char, style: FontStyle) -> Typeface {
    let s_key = style_to_key(style);
    FALLBACK_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(tf) = cache.get(&(c, s_key)) {
            return tf.clone();
        }
        
        let mgr = get_font_mgr();
        let tf = mgr.match_family_style_character("", style, &["zh-CN", "ja-JP", "en-US"], c as i32)
            .unwrap_or_else(|| mgr.legacy_make_typeface(None, style).unwrap());
        
        cache.insert((c, s_key), tf.clone());
        tf
    })
}

fn draw_text_optimized(canvas: &Canvas, text: &str, pos: (f32, f32), size: f32, style: FontStyle, paint: &Paint) {
    let mut current_x = pos.0.round();
    let y = pos.1.round();
    
    let mut group_text = String::new();
    let mut last_tf: Option<Typeface> = None;

    for c in text.chars() {
        let tf = get_typeface_for_char(c, style);
        
        if let Some(ref ltf) = last_tf {
            if ltf.unique_id() != tf.unique_id() {
                let font = Font::from_typeface(ltf.clone(), size);
                canvas.draw_str(&group_text, (current_x, y), &font, paint);
                let (w, _) = font.measure_str(&group_text, None);
                current_x += w;
                group_text.clear();
            }
            group_text.push(c);
        } else {
            group_text.push(c);
        }
        last_tf = Some(tf);
    }

    if let Some(ltf) = last_tf {
        let font = Font::from_typeface(ltf, size);
        canvas.draw_str(&group_text, (current_x, y), &font, paint);
    }
}

pub fn draw_main_page(canvas: &Canvas, ox: f32, oy: f32, w: f32, h: f32, alpha: u8, media: &MediaInfo) {
    draw_arrow_right(canvas, ox + w - 20.0, oy + h / 2.0, alpha);

    if media.title.is_empty() {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Color::from_argb((alpha as f32 * 0.5) as u8, 255, 255, 255));
        draw_text_optimized(canvas, "No Media Playing", (ox + 30.0, oy + 45.0), 13.0, FontStyle::normal(), &paint);
        return;
    }

    let card_w = 180.0;
    let card_h = 56.0;
    let card_x = ox + 20.0;
    let card_y = oy + 16.0;

    let mut card_paint = Paint::default();
    card_paint.set_anti_alias(true);
    card_paint.set_color(Color::from_argb((alpha as f32 * 0.3) as u8, 40, 40, 45));
    canvas.draw_round_rect(Rect::from_xywh(card_x, card_y, card_w, card_h), 14.0, 14.0, &card_paint);

    let mut border_paint = Paint::default();
    border_paint.set_anti_alias(true);
    border_paint.set_style(skia_safe::paint::Style::Stroke);
    border_paint.set_stroke_width(0.5);
    border_paint.set_color(Color::from_argb((alpha as f32 * 0.1) as u8, 255, 255, 255));
    canvas.draw_round_rect(Rect::from_xywh(card_x, card_y, card_w, card_h), 14.0, 14.0, &border_paint);

    let img_size = 40.0;
    let img_x = card_x + 8.0;
    let img_y = card_y + 8.0;

    let cache_key = format!("{}-{}", media.title, media.album);
    let mut image_to_draw = None;

    IMG_CACHE.with(|cache| {
        let mut cache_mut = cache.borrow_mut();
        if let Some((key, img)) = cache_mut.as_ref() {
            if key == &cache_key { image_to_draw = Some(img.clone()); return; }
        }
        if let Some(ref bytes) = media.thumbnail {
            let data = Data::new_copy(bytes);
            if let Some(image) = Image::from_encoded(data) {
                *cache_mut = Some((cache_key.clone(), image.clone()));
                image_to_draw = Some(image);
            }
        }
    });

    canvas.save();
    canvas.clip_rrect(RRect::new_rect_xy(Rect::from_xywh(img_x, img_y, img_size, img_size), 10.0, 10.0), skia_safe::ClipOp::Intersect, true);
    if let Some(img) = image_to_draw {
        let mut img_paint = Paint::default();
        img_paint.set_anti_alias(true);
        img_paint.set_alpha_f(alpha as f32 / 255.0);
        canvas.draw_image_rect_with_sampling_options(
            img, None, Rect::from_xywh(img_x, img_y, img_size, img_size),
            SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear), &img_paint
        );
    } else {
        draw_placeholder(canvas, img_x, img_y, img_size, alpha);
    }
    canvas.restore();

    let text_x = img_x + img_size + 12.0;
    let max_text_w = card_w - (img_size + 32.0);
    let text_start_y = card_y + 22.0;

    let mut text_paint = Paint::default();
    text_paint.set_anti_alias(true);

    text_paint.set_color(Color::from_argb(alpha, 255, 255, 255));
    let title_disp = get_truncated_text(&media.title, max_text_w, 13.0, FontStyle::bold());
    draw_text_optimized(canvas, &title_disp, (text_x, text_start_y), 13.0, FontStyle::bold(), &text_paint);

    text_paint.set_color(Color::from_argb((alpha as f32 * 0.6) as u8, 255, 255, 255));
    let artist_disp = get_truncated_text(&media.artist, max_text_w, 11.0, FontStyle::normal());
    draw_text_optimized(canvas, &artist_disp, (text_x, text_start_y + 18.0), 11.0, FontStyle::normal(), &text_paint);
}

fn get_truncated_text(text: &str, max_w: f32, size: f32, style: FontStyle) -> String {
    let key = format!("{}-{}-{:?}-{}", text, max_w, style, size);
    TEXT_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(t) = cache.get(&key) {
            return t.clone();
        }
        
        let mut current_w = 0.0;
        let mut result = String::new();
        for c in text.chars() {
            let tf = get_typeface_for_char(c, style);
            let font = Font::from_typeface(tf, size);
            let (w, _) = font.measure_str(&c.to_string(), None);
            if current_w + w > max_w - 10.0 {
                result.push_str("...");
                break;
            }
            current_w += w;
            result.push(c);
        }
        cache.insert(key, result.clone());
        result
    })
}

fn draw_placeholder(canvas: &Canvas, x: f32, y: f32, size: f32, alpha: u8) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb((alpha as f32 * 0.1) as u8, 255, 255, 255));
    canvas.draw_round_rect(Rect::from_xywh(x, y, size, size), 10.0, 10.0, &paint);
    let cx = x + size/2.0; let cy = y + size/2.0;
    paint.set_color(Color::from_argb((alpha as f32 * 0.4) as u8, 255, 255, 255));
    let mut builder = PathBuilder::new();
    builder.move_to(Point::new(cx - 3.0, cy + 5.0));
    builder.line_to(Point::new(cx - 3.0, cy - 6.0));
    builder.line_to(Point::new(cx + 4.0, cy - 8.0));
    builder.line_to(Point::new(cx + 4.0, cy + 3.0));
    builder.close();
    canvas.draw_path(&builder.detach(), &paint);
    canvas.draw_circle(Point::new(cx - 5.5, cy + 5.0), 2.5, &paint);
    canvas.draw_circle(Point::new(cx + 1.5, cy + 3.0), 2.5, &paint);
}
