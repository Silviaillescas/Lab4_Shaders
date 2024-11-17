mod render;
mod shaders;
mod camera;
mod model; // Importar el módulo `model`

use minifb::{Key, Window, WindowOptions};
use render::render;
use camera::Camera;
use model::Model;
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
    let mut camera = Camera::new(); // Crear instancia de la cámara

    // Cargar el modelo de la nave desde el archivo `nave.obj`
    let model = Model::load_from_obj("nave.obj");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed_time = start_time.elapsed().as_secs_f32();

        // Control de la cámara en los ejes X e Y
        if window.is_key_down(Key::A) {
            camera.move_camera("left");
        }
        if window.is_key_down(Key::D) {
            camera.move_camera("right");
        }
        if window.is_key_down(Key::W) {
            camera.move_camera("up");
        }
        if window.is_key_down(Key::S) {
            camera.move_camera("down");
        }

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

        // Renderizar la escena, ahora pasando la cámara y el modelo
        render(&mut buffer, key, elapsed_time, &camera, &model);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
