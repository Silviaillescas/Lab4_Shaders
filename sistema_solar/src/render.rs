use minifb::Key;
use nalgebra::Vector3;
use crate::shaders::{star_shader, rocky_planet_shader, gas_giant_shader, ring_shader, moon_shader};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn render(buffer: &mut [u32], key: Option<Key>, time: f32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Convertimos las coordenadas de pantalla a una escala de 0.0 a 1.0
            let position = Vector3::new(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32, 0.0);
            
            // Seleccionamos el shader según la tecla presionada
            let color = match key {
                Some(Key::Key1) => star_shader(position),                      // Estrella
                Some(Key::Key2) => rocky_planet_shader(position, time),        // Planeta rocoso con efecto 3D
                Some(Key::Key3) => gas_giant_shader(position, time),           // Gigante gaseoso
                Some(Key::Key4) => ring_shader(position),                      // Anillos
                Some(Key::Key5) => moon_shader(position, time),                // Luna
                _ => Vector3::new(0.0, 0.0, 0.0),                              // Fondo negro si no hay selección
            };

            // Convertimos el color a un formato de 32 bits y lo asignamos al buffer
            buffer[y * WIDTH + x] = color_to_u32(color);
        }
    }
}

// Función para convertir Vector3<f32> a un color u32 compatible con el buffer
fn color_to_u32(color: Vector3<f32>) -> u32 {
    let r = (color.x * 255.0) as u32;
    let g = (color.y * 255.0) as u32;
    let b = (color.z * 255.0) as u32;
    (r << 16) | (g << 8) | b
}
