use minifb::Key;
use nalgebra::Vector3;
use crate::shaders::{
    emissive_star_shader, rocky_planet_shader, gas_giant_shader, ring_shader, moon_shader,
    asteroid_shader,
};
use crate::camera::Camera;
use crate::model::Model;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn render(buffer: &mut [u32], key: Option<Key>, time: f32, camera: &Camera, model: &Model) {
    // Limpiar el buffer con un color de fondo negro
    buffer.iter_mut().for_each(|pixel| *pixel = 0x000000);

    if let Some(Key::Key6) = key {
        // Renderizar la nave modelada
        println!("Renderizando la nave..."); // Mensaje de depuración
        for i in 0..model.mesh.indices.len() / 3 {
            let idx0 = model.mesh.indices[3 * i] as usize;
            let idx1 = model.mesh.indices[3 * i + 1] as usize;
            let idx2 = model.mesh.indices[3 * i + 2] as usize;

            let v0 = Vector3::new(
                model.mesh.positions[3 * idx0] * 100.0, // Ajuste de escala
                model.mesh.positions[3 * idx0 + 1] * 100.0, // Ajuste de escala
                model.mesh.positions[3 * idx0 + 2] * 100.0, // Ajuste de escala
            );

            let v1 = Vector3::new(
                model.mesh.positions[3 * idx1] * 100.0, // Ajuste de escala
                model.mesh.positions[3 * idx1 + 1] * 100.0, // Ajuste de escala
                model.mesh.positions[3 * idx1 + 2] * 100.0, // Ajuste de escala
            );

            let v2 = Vector3::new(
                model.mesh.positions[3 * idx2] * 100.0, // Ajuste de escala
                model.mesh.positions[3 * idx2 + 1] * 100.0, // Ajuste de escala
                model.mesh.positions[3 * idx2 + 2] * 100.0, // Ajuste de escala
            );

            // Transformar según la cámara y renderizar los triángulos
            let vertices = [v0, v1, v2];
            for v in &vertices {
                let x = ((v.x + camera.position.x) * WIDTH as f32 / 2.0) as usize;
                let y = ((v.y + camera.position.y) * HEIGHT as f32 / 2.0) as usize;
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = 0xffffff; // Dibujar un punto blanco
                }
            }
        }
    } else {
        // Renderizado de otros objetos según el shader correspondiente
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let position = Vector3::new(
                    (x as f32 / WIDTH as f32) + camera.position.x,
                    (y as f32 / HEIGHT as f32) + camera.position.y,
                    0.0,
                );

                let color = match key {
                    Some(Key::Key1) => emissive_star_shader(position),
                    Some(Key::Key2) => rocky_planet_shader(position, time),
                    Some(Key::Key3) => gas_giant_shader(position, time),
                    Some(Key::Key4) => ring_shader(position),
                    Some(Key::Key5) => moon_shader(position, time),
                    Some(Key::Key6) => asteroid_shader(position), // Asteroide, pero vamos a usar la nave también.
                    _ => Vector3::new(0.0, 0.0, 0.0), // Fondo negro
                };

                buffer[y * WIDTH + x] = color_to_u32(color);
            }
        }
    }
}

fn color_to_u32(color: Vector3<f32>) -> u32 {
    let r = (color.x * 255.0).min(255.0).max(0.0) as u32;
    let g = (color.y * 255.0).min(255.0).max(0.0) as u32;
    let b = (color.z * 255.0).min(255.0).max(0.0) as u32;
    (r << 16) | (g << 8) | b
}
