use nalgebra::Vector3;

// Genera ruido simple (Perlin o similar) basado en la posición
fn simple_noise(value: f32) -> f32 {
    (value.sin() * 43758.5453).fract()
}

pub fn star_shader(position: Vector3<f32>) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0);
    let distance = (position - center).magnitude();
    let intensity = (1.0 - distance * 2.0).max(0.0);
    let star_color = Vector3::new(1.0, 0.9, 0.5);
    star_color * intensity
}

pub fn rocky_planet_shader(position: Vector3<f32>) -> Vector3<f32> {
    // Definimos el centro del planeta en el centro de la pantalla
    let center = Vector3::new(0.5, 0.5, 0.0);
    let distance = (position - center).magnitude();

    // Tamaño del planeta
    let planet_radius = 0.3;

    // Si está fuera del planeta, devolvemos un color negro
    if distance > planet_radius {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    // Generamos un valor de ruido para simular continentes
    let noise = simple_noise(position.x * 10.0 + position.y * 10.0);

    // Definimos los colores base para continentes y océanos
    let land_color = Vector3::new(0.4, 0.6, 0.2);  // Verde
    let ocean_color = Vector3::new(0.0, 0.3, 0.5);  // Azul

    // Mezclamos los colores basándonos en el ruido para crear el efecto de continentes
    land_color * noise + ocean_color * (1.0 - noise)
}
