mod camera;
mod spaceship;
mod render;
mod shaders;

use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use camera::Camera;
use spaceship::Spaceship;
use render::{render, render_skybox};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut camera = Camera::new();
    
    // Puntos de warp predefinidos
    let warp_points = [
        (0.0, 0.0, -20.0),
        (20.0, 0.0, -30.0),
        (-20.0, 10.0, -25.0),
        (0.0, -15.0, -40.0),
    ];

    // Cargar la nave desde el archivo .obj
    let spaceship = Spaceship::load_from_obj("nave.obj")
        .expect("Error cargando la nave");

    let start_time: Instant = Instant::now();
    let mut warp_target: Option<(f32, f32, f32)> = None;
    let mut warp_progress: f32 = 0.0; // Progreso del warp, entre 0.0 y 1.0

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
                Key::Key1 => {
                    warp_target = Some(warp_points[0]);
                    warp_progress = 0.0; // Reiniciar el progreso del warp
                },
                Key::Key2 => {
                    warp_target = Some(warp_points[1]);
                    warp_progress = 0.0;
                },
                Key::Key3 => {
                    warp_target = Some(warp_points[2]);
                    warp_progress = 0.0;
                },
                Key::Key4 => {
                    warp_target = Some(warp_points[3]);
                    warp_progress = 0.0;
                },
                Key::R => {
                    // Restablecer la cámara a la posición inicial
                    camera = Camera::new(); // Reestablecer la posición de la cámara y el FOV
                    warp_target = None; // Cancelar cualquier objetivo de warp
                    warp_progress = 0.0;
                },
                _ => {}
            }
        }

        // Realizar el warp si hay un destino definido
        if let Some(target) = warp_target {
            if warp_progress < 1.0 {
                warp_progress += 0.03; // Ajustar el progreso para ser más suave
                camera.animate_to(target, 0.07, warp_progress); // Velocidad moderada del warp
            } else {
                warp_target = None; // Detener el warp cuando se llegue al destino
            }
        }

        // Renderizar el skybox primero
        render_skybox(&mut buffer);

        // Efecto de oscurecimiento reducido durante el warp
        if warp_target.is_some() {
            let fade_intensity = 0.3 + 0.7 * (1.0 - warp_progress).max(0.0); // Menos oscuro para mantener la visibilidad
            buffer.iter_mut().for_each(|pixel| {
                *pixel = ((*pixel as f32) * fade_intensity) as u32; // Oscurecer pantalla pero mantener elementos visibles
            });
        }

        // Renderizar la escena con la nave y otros objetos
        render(&mut buffer, elapsed_time, &mut camera, &spaceship);

        // Actualizar la ventana con el contenido del buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
