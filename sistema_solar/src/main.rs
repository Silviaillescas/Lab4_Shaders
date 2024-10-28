mod render;
mod shaders;

use minifb::{Key, Window, WindowOptions};
use render::render;

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Captura la tecla presionada
        let key = if window.is_key_down(Key::Key1) {
            Some(Key::Key1)
        } else if window.is_key_down(Key::Key2) {
            Some(Key::Key2)
        } else {
            None
        };

        // Llama a la función de renderizado
        render(&mut buffer, key);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
