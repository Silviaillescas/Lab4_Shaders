mod camera;
mod spaceship;
mod render;
mod shaders;

use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use camera::Camera;
use spaceship::Spaceship;
use render::{render, render_skybox}; // Asegúrate de que `render_skybox` esté disponible

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut camera = Camera::new();
    
    // Cargar la nave desde el archivo .obj
    let spaceship = Spaceship::load_from_obj("nave.obj")
        .expect("Error cargando la nave");

    let start_time: Instant = Instant::now();

    // Crear la ventana principal
    let mut window = Window::new(
        "Sistema Solar - Nave Modelada",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Obtener el tiempo transcurrido
        let elapsed_time: f32 = start_time.elapsed().as_secs_f32();
    
        // Obtener todas las teclas actualmente presionadas
        let keys: Vec<Key> = window.get_keys();      
        
        // Verificar teclas y mover la cámara
        for key in keys.iter() {
            match key {
                Key::W => camera.move_camera("forward"),
                Key::S => camera.move_camera("backward"),
                Key::A => camera.move_camera("left"),
                Key::D => camera.move_camera("right"),
                _ => {}
            }
        }
    
        // Renderizar el skybox primero
        render_skybox(&mut buffer);

        // Renderizar la escena con la nave y otros objetos
        render(&mut buffer, elapsed_time, &mut camera, &spaceship);

        // Actualizar la ventana con el contenido del buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
