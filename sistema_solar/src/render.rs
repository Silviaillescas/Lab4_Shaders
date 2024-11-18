use minifb::Key;
use nalgebra::{Matrix4, Point3, Vector3};
use crate::shaders::{emissive_star_shader, rocky_planet_shader, gas_giant_shader, ring_shader, moon_shader};
use crate::camera::Camera;
use crate::spaceship::Spaceship;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn render(buffer: &mut [u32], time: f32, camera: &mut Camera, spaceship: &Spaceship) {
    // Limpiar el buffer con un color de fondo azul oscuro
    buffer.iter_mut().for_each(|pixel| *pixel = 0x000080); // Azul oscuro

    // Renderizar la nave modelada dibujando puntos verdes
    println!("Renderizando la nave modelada...");

    // Configurar una proyección en perspectiva para el modelo
    let aspect_ratio: f32 = WIDTH as f32 / HEIGHT as f32;
    let perspective = Matrix4::new_perspective(aspect_ratio, 45.0_f32.to_radians(), 0.1, 100.0);
    let view_matrix = camera.get_view_matrix();
    let transformation_matrix = perspective * view_matrix;

    let scale_factor: f32 = 1.0; // Escala ajustada para mejor visualización
    let translation_vector = Vector3::new(0.0, 0.0, -10.0); // Ajuste de posición para que el modelo esté más cerca de la cámara

    // Recorrer los vértices y renderizarlos
    for v in &spaceship.vertices {
        // Escalar y trasladar los vértices para mayor visibilidad
        let v = *v * scale_factor + translation_vector;

        // Transformar cada vértice usando la matriz de transformación
        let transformed_v = transformation_matrix.transform_point(&v);

        // Dibujar el vértice si está dentro del rango de visión
        if transformed_v.z > 0.0 {
            let x: isize = ((transformed_v.x / transformed_v.z) * (WIDTH as f32 / 2.0) + (WIDTH as f32 / 2.0)) as isize;
            let y: isize = ((-transformed_v.y / transformed_v.z) * (HEIGHT as f32 / 2.0) + (HEIGHT as f32 / 2.0)) as isize;

            // Verificar si las coordenadas están dentro de los límites de la pantalla
            if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                let index = (y as usize) * WIDTH + (x as usize);
                if index < buffer.len() {
                    buffer[index] = 0x00ff00; // Dibujar un punto verde
                }
            }
        }
    }

    // Renderizar todos los objetos del sistema solar sin necesidad de presionar teclas
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let position = Vector3::new(
                (x as f32 / WIDTH as f32) * 2.0 - 1.0 + camera.position.x,
                (y as f32 / HEIGHT as f32) * 2.0 - 1.0 + camera.position.y,
                0.0,
            );

            // Aplicar cada uno de los shaders para diferentes tipos de objetos
            let color_star = emissive_star_shader(position);
            let color_rocky = rocky_planet_shader(position, time);
            let color_gas_giant = gas_giant_shader(position, time);
            let color_ring = ring_shader(position);
            let color_moon = moon_shader(position, time);

            // Combinar los colores sumándolos, limitando el valor máximo para evitar saturación
            let combined_color = Vector3::new(
                (color_star.x + color_rocky.x + color_gas_giant.x + color_ring.x + color_moon.x).min(1.0),
                (color_star.y + color_rocky.y + color_gas_giant.y + color_ring.y + color_moon.y).min(1.0),
                (color_star.z + color_rocky.z + color_gas_giant.z + color_ring.z + color_moon.z).min(1.0),
            );

            let index = y * WIDTH + x;
            buffer[index] = color_to_u32(combined_color);
        }
    }
}

fn color_to_u32(color: Vector3<f32>) -> u32 {
    let r = (color.x * 255.0).min(255.0).max(0.0) as u32;
    let g = (color.y * 255.0).min(255.0).max(0.0) as u32;
    let b = (color.z * 255.0).min(255.0).max(0.0) as u32;
    (r << 16) | (g << 8) | b
}
