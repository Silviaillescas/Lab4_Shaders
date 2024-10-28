mod render;
mod shaders;

use minifb::{Key, Window, WindowOptions};
use render::render;
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new(
        "Sistema Solar - Estrella y Planeta Rocoso",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("No se pudo crear la ventana: {}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed_time = start_time.elapsed().as_secs_f32();
        
        let key = if window.is_key_down(Key::Key1) {
            Some(Key::Key1)
        } else if window.is_key_down(Key::Key2) {
            Some(Key::Key2)
        } else if window.is_key_down(Key::Key3) {
            Some(Key::Key3)
        } else if window.is_key_down(Key::Key4) {
            Some(Key::Key4)
        } else if window.is_key_down(Key::Key5) {
            Some(Key::Key5)
        } else {
            None
        };

        render(&mut buffer, key, elapsed_time);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
