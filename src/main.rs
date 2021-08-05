use piston_window::*;

fn main() {
    
    // gotta create a vector of rgba values multidimensional to the size of the screen

    let width = 200.0;
    let height = 200.0;

    let background_color = [255u8, 255, 255, 255];
    let mut active_color = [0u8, 0, 0, 255];

    let mut image1: Vec<u8> = Vec::new();

    for i2 in 0..width as u32 {
        for i in 0..height as u32 {
            for e in background_color {
                image1.push(e);
            }            
        }
    }

    let size = Size {
                width,
                height,
            };
    let mut brush_size = Size {
        width: 1.0,
        height: 1.0,
    };

    let mut window: PistonWindow = WindowSettings::new(
            "piston: hello_world",
            size,            
        )
        .exit_on_esc(true)
        .vsync(false)
        .build()
        .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let mut texture = Texture::from_memory_alpha(
                        &mut texture_context, 
                        &image1,
                        size.width as u32,
                        size.height as u32,
                        &TextureSettings::new()).unwrap();

    let mut glyphs = window.load_font("assets/FiraSans-Regular.ttf").unwrap();

    let mut mc = [0.0, 0.0];
    window.set_lazy(true);
    while let Some(e) = window.next() {
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

            image(&texture, transform, g);
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        if let Some(mouse_cursor) = e.mouse_cursor_args() {
            mc = mouse_cursor;
        }
        
        match e.button_args()
            {
            Some(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Mouse(MouseButton::Left),
                ..
            }) => {
                texture::UpdateTexture::update(
                    &mut texture,
                    &mut texture_context,
                    texture::Format::Rgba8,
                    &active_color[..],
                    [mc[0] as u32, mc[1] as u32],
                    brush_size
                );
            },
            _ => {}
        }

    }
}