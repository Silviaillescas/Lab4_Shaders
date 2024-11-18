use nalgebra::{Matrix4, Vector3};
use crate::shaders::{emissive_star_shader, rocky_planet_shader, gas_giant_shader, ring_shader, moon_shader};
use crate::camera::Camera;
use crate::spaceship::Spaceship;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// La función render ahora incluye el renderizado de órbitas
pub fn render(buffer: &mut [u32], time: f32, camera: &mut Camera, spaceship: &Spaceship) {
    // Renderizar el skybox primero
    render_skybox(buffer);

    // Renderizar las órbitas de los planetas
    render_orbits(buffer);

    // Renderizar la nave modelada dibujando puntos verdes
    let aspect_ratio: f32 = WIDTH as f32 / HEIGHT as f32;
    let perspective = Matrix4::new_perspective(aspect_ratio, 45.0_f32.to_radians(), 0.1, 100.0);
    let view_matrix = camera.get_view_matrix();
    let transformation_matrix = perspective * view_matrix;

    let scale_factor: f32 = 1.0;
    let translation_vector = Vector3::new(0.0, 0.0, -10.0);

    for v in &spaceship.vertices {
        let v = *v * scale_factor + translation_vector;
        let transformed_v = transformation_matrix.transform_point(&v);

        if transformed_v.z > 0.0 {
            let x: isize = ((transformed_v.x / transformed_v.z) * (WIDTH as f32 / 2.0) + (WIDTH as f32 / 2.0)) as isize;
            let y: isize = ((-transformed_v.y / transformed_v.z) * (HEIGHT as f32 / 2.0) + (HEIGHT as f32 / 2.0)) as isize;

            if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                let index = (y as usize) * WIDTH + (x as usize);
                if index < buffer.len() {
                    buffer[index] = 0x00ff00; // Dibujar un punto verde (nave)
                }
            }
        }
    }

    // Renderizar todos los objetos del sistema solar
    let positions = [
        Vector3::new(-0.5, 0.0, 0.0),  // Emissive Star
        Vector3::new(0.5, 0.0, 0.0),   // Rocky Planet
        Vector3::new(0.0, 0.5, 0.0),   // Gas Giant
        Vector3::new(0.0, -0.5, 0.0),  // Ring
        Vector3::new(-0.5, -0.5, 0.0), // Moon
    ];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let position = Vector3::new(
                (x as f32 / WIDTH as f32) * 2.0 - 1.0 + camera.position.x,
                (y as f32 / HEIGHT as f32) * 2.0 - 1.0 + camera.position.y,
                0.0,
            );

            let mut combined_color = Vector3::new(0.0, 0.0, 0.0);

            combined_color += emissive_star_shader(position + positions[0]);
            combined_color += rocky_planet_shader(position + positions[1], time);
            combined_color += gas_giant_shader(position + positions[2], time);
            combined_color += ring_shader(position + positions[3]);
            combined_color += moon_shader(position + positions[4], time);

            combined_color = Vector3::new(
                combined_color.x.min(1.0),
                combined_color.y.min(1.0),
                combined_color.z.min(1.0),
            );

            let index = y * WIDTH + x;
            if buffer[index] == color_to_u32(Vector3::new(0.0, 0.0, 0.1)) {
                // Solo sobrescribir si el color en el buffer es el del skybox (fondo)
                buffer[index] = color_to_u32(combined_color);
            }
        }
    }
}

// Nueva función para renderizar las órbitas de los planetas mejorada
pub fn render_orbits(buffer: &mut [u32]) {
    let center_x = WIDTH as f32 / 2.0;
    let center_y = HEIGHT as f32 / 2.0;

    // Dibujar varias órbitas circulares con mayor densidad y grosor
    let radii = [100.0, 150.0, 200.0]; // Radios de las órbitas para cada planeta
    for &radius in &radii {
        for angle in (0..360).step_by(1) {  // Reducimos el paso a `1` para que haya más puntos dibujando cada órbita
            let theta = (angle as f32).to_radians();
            let x = center_x + radius * theta.cos();
            let y = center_y + radius * theta.sin();

            // Dibujar un grosor adicional para cada órbita
            for offset in -1..=1 {
                let x_offset = x + (offset as f32);
                let y_offset = y + (offset as f32);

                if x_offset >= 0.0 && x_offset < WIDTH as f32 && y_offset >= 0.0 && y_offset < HEIGHT as f32 {
                    let index = (y_offset as usize) * WIDTH + (x_offset as usize);
                    if index < buffer.len() {
                        buffer[index] = 0xFFFFFF; // Dibujar un punto blanco brillante para la órbita
                    }
                }
            }
        }
    }
}

// Nueva función para renderizar el skybox con estrellas menos densas y más sutiles
pub fn render_skybox(buffer: &mut [u32]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let normalized_x = (x as f32 / WIDTH as f32) * 2.0 - 1.0;
            let normalized_y = (y as f32 / HEIGHT as f32) * 2.0 - 1.0;
            let position = Vector3::new(normalized_x, normalized_y, 0.0);

            // Renderizar estrellas con mayor visibilidad
            let color = skybox_shader(position, x, y);
            let index = y * WIDTH + x;
            buffer[index] = color_to_u32(color);
        }
    }
}

// Shader para el skybox con estrellas menos densas y más sutiles
pub fn skybox_shader(position: Vector3<f32>, x: usize, y: usize) -> Vector3<f32> {
    // Reducir la densidad de las estrellas
    let star_density = 50.0;  // Valor reducido para hacer que haya menos estrellas
    let noise = ((x as f32 * 12.9898 + y as f32 * 78.233).sin() * 43758.5453).fract().abs();

    // Hacer que las estrellas sean menos frecuentes con un umbral más alto
    if noise > 0.98 {
        Vector3::new(1.0, 1.0, 1.0) * 0.5 // Estrella blanca brillante pero con menor intensidad
    } else {
        // Fondo azul oscuro uniforme para el cielo nocturno
        Vector3::new(0.0, 0.0, 0.1) // Azul oscuro para el fondo del cielo
    }
}

pub fn color_to_u32(color: Vector3<f32>) -> u32 {
    let r = (color.x * 255.0).min(255.0).max(0.0) as u32;
    let g = (color.y * 255.0).min(255.0).max(0.0) as u32;
    let b = (color.z * 255.0).min(255.0).max(0.0) as u32;
    (r << 16) | (g << 8) | b
}
