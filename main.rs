use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use rusttype::{Font, Scale, Point};

fn render_text<T>(                       // function generic over T
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<T>, // T is the generic type
    font: &Font<'static>,
    text: &str,
    scale: Scale,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), String> {
    let _v_metrics = font.v_metrics(scale);
    let y_offset = y + _v_metrics.ascent as i32;
    let mut x_offset = x;

    for c in text.chars() {
        let positioned_glyph = font.glyph(c).scaled(scale).positioned(Point { x: x_offset as f32, y: y_offset as f32 });

        if let Some(bb) = positioned_glyph.pixel_bounding_box() {
            let glyph_width = (bb.max.x - bb.min.x) as u32;
            let glyph_height = (bb.max.y - bb.min.y) as u32;

            if glyph_width > 0 && glyph_height > 0 {
                let mut glyph_texture = texture_creator
                    .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGBA8888, glyph_width, glyph_height)
                    .map_err(|e| e.to_string())?;

                glyph_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    positioned_glyph.draw(|x, y, v| {
                        let index = (y as usize * pitch + x as usize * 4) as usize;
                        buffer[index] = (color.r as f32 * v) as u8; // Apply color
                        buffer[index + 1] = (color.g as f32 * v) as u8;
                        buffer[index + 2] = (color.b as f32 * v) as u8;
                        buffer[index + 3] = 255; // Alpha
                    });
                })?;

                let target_y = y_offset - glyph_height as i32;
                let target = Rect::new(x_offset, target_y, glyph_width, glyph_height);
                canvas.copy(&glyph_texture, None, Some(target))?;
            }
        }
        x_offset += font.glyph(c).scaled(scale).h_metrics().advance_width as i32;
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Texter", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Create the texture creator BEFORE calling render_text
    let texture_creator = canvas.texture_creator();

    let font_data: &[u8] = include_bytes!("../assets/FiraCode-Bold.ttf");
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    let text = "Hello, alkis!";
    let scale = Scale::uniform(32.0);
    let color = Color::RGB(255, 255, 255); // text color

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    render_text(&mut canvas, &texture_creator, &font, text, scale, 100, 100, color)?;

    canvas.present();
    loop {}
}