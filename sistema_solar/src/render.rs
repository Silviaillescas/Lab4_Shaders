use minifb::Key;
use nalgebra::Vector3;
use crate::shaders::{star_shader, rocky_planet_shader};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn render(buffer: &mut [u32], key: Option<Key>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Convertimos las coordenadas de pantalla a una escala de 0.0 a 1.0
            let position = Vector3::new(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32, 0.0);
            let color = match key {
                Some(Key::Key1) => star_shader(position),       // Presiona '1' para la estrella
                Some(Key::Key2) => rocky_planet_shader(position), // Presiona '2' para el planeta rocoso
                _ => Vector3::new(0.0, 0.0, 0.0),                // Fondo negro si no hay selección
            };
            buffer[y * WIDTH + x] = color_to_u32(color);
        }
    }
}

fn color_to_u32(color: Vector3<f32>) -> u32 {
    let r = (color.x * 255.0) as u32;
    let g = (color.y * 255.0) as u32;
    let b = (color.z * 255.0) as u32;
    (r << 16) | (g << 8) | b
}
