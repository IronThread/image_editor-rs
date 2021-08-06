use piston_window::*;

use image2::{ImageBuffer, Rgba};

fn main() {
    let width = 200.0;
    let height = 200.0;

    let background_color = [255u8, 255, 255, 255];
    let mut active_color = [0u8, 0, 0, 255];

    let mut image1: Vec<u8> = Vec::with_capacity((width as usize * height as usize) * 4);

    for i2 in 0..width as u32 {
        for i in 0..height as u32 {
            image1.extend(background_color.iter().copied());
        }
    }

    let mut image1 =
        <ImageBuffer<Rgba<u8>, Vec<u8>>>::from_vec(width as u32, height as u32, image1).unwrap();
    let mut should_paint = false;

    let size = Size { width, height };
    let mut brush_size = 4.0;

    let mut window: PistonWindow = WindowSettings::new("image editor", size)
        .exit_on_esc(true)
        .vsync(false)
        .build()
        .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let mut glyphs = window.load_font("assets/FiraSans-Regular.ttf").unwrap();

    let mut mc = [0.0, 0.0];
    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(mouse_cursor) = e.mouse_cursor_args() {
            mc = mouse_cursor;
            if should_paint {
                for w in (mc[0] as u32..mc[0] as u32 + brush_size as u32)
                {
                    for h in (mc[1] as u32..mc[1] as u32 + brush_size as u32) {
                        if w < width as u32 && h < height as u32 {
                            image1.put_pixel(w, h, Rgba(active_color));
                        }
                    }
                }
            }
        }

        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(0.0, 0.0);

            clear([0.0, 0.0, 0.0, 1.0], g);

            /*
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Hello world!",
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
            */

            let texture =
                Texture::from_image(&mut texture_context, &image1, &TextureSettings::new())
                    .unwrap();

            image(&texture, transform, g);
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        match e.button_args() {
            Some(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Mouse(MouseButton::Left),
                ..
            }) => {
                should_paint = true;
            }
            Some(ButtonArgs {
                state: ButtonState::Release,
                button: Button::Mouse(MouseButton::Left),
                ..
            }) => {
                should_paint = false;
            }
            _ => {}
        }
    }
}
