use nalgebra::Vector3;

// Función de ruido simple para crear variación de textura
fn simple_noise(x: f32, y: f32) -> f32 {
    ((x * 10.0).sin() + (y * 10.0).cos()).abs() * 0.5
}

// Shader para la estrella (sol) en el centro del sistema
pub fn star_shader(position: Vector3<f32>) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0); // Centro de la estrella en el medio de la pantalla
    let distance = (position - center).magnitude();
    let star_radius = 0.2; // Tamaño de la estrella

    // Determina el brillo de la estrella basado en la distancia al centro
    if distance > star_radius {
        return Vector3::new(0.0, 0.0, 0.0); // Fondo negro fuera de la estrella
    }

    // Crear un efecto de brillo
    let intensity = (1.0 - distance / star_radius).powf(2.0); // Ajusta la caída de brillo
    let star_color = Vector3::new(1.0, 0.9, 0.5); // Color de la estrella (amarillo claro)

    star_color * intensity
}

pub fn rocky_planet_shader(position: Vector3<f32>, time: f32) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0); // Centro del planeta
    let planet_radius = 0.3; // Radio del planeta

    // Calcula la distancia desde el centro del planeta
    let distance = (position - center).magnitude();
    if distance > planet_radius {
        return Vector3::new(0.0, 0.0, 0.0); // Fuera del planeta, fondo negro
    }

    // Proyecta la posición en un espacio 3D simulando una esfera
    let sphere_normal = (position - center).normalize();

    // Dirección de la luz ajustada para un ángulo más uniforme
    let light_dir = Vector3::new(0.4, -0.3, 0.7).normalize();

    // Calcular iluminación difusa con un valor mínimo para evitar sombras intensas
    let diff_intensity = sphere_normal.dot(&light_dir).max(0.5); // Ajuste para que no sea demasiado oscuro

    // Suavizar el ruido para continentes y océanos
    let noise = (simple_noise(position.x * 1.5, position.y * 1.5) * 0.5 + 0.5).powf(1.5); // Suavizado del ruido
    let ocean_color = Vector3::new(0.0, 0.3, 0.6); // Azul más claro para océanos
    let land_color = Vector3::new(0.5, 0.7, 0.4); // Verde para continentes
    let color = land_color * noise + ocean_color * (1.0 - noise);

    // Efecto de atmósfera con halo más sutil y suavizado
    let cloud_layer = simple_noise(position.x + time * 0.05, position.y + time * 0.05) * 0.3;
    let atmosphere_color = Vector3::new(0.9, 0.9, 1.0) * cloud_layer;

    // Borde de la atmósfera para el efecto de halo
    let halo_strength = (1.0 - (distance / planet_radius).powf(3.0)).max(0.0);
    let halo_color = atmosphere_color * halo_strength;

    // Aplicar iluminación difusa al color y mezclar con el halo
    let final_color = color * diff_intensity + halo_color;

    final_color
}


// Shader para un gigante gaseoso con bandas
pub fn gas_giant_shader(position: Vector3<f32>, time: f32) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0);
    let distance = (position - center).magnitude();
    let planet_radius = 0.35;

    if distance > planet_radius {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let band_pattern = ((position.y * 20.0 + time * 0.1).sin() * 0.5 + 0.5).abs();
    let color1 = Vector3::new(0.7, 0.4, 0.2);
    let color2 = Vector3::new(0.3, 0.6, 0.7);

    color1 * band_pattern + color2 * (1.0 - band_pattern)
}

// Shader para los anillos del gigante gaseoso
pub fn ring_shader(position: Vector3<f32>) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0);
    let distance = (position - center).magnitude();
    let ring_inner_radius = 0.4;
    let ring_outer_radius = 0.5;

    if distance < ring_inner_radius || distance > ring_outer_radius {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let ring_color1 = Vector3::new(0.7, 0.7, 0.7);
    let ring_color2 = Vector3::new(0.4, 0.4, 0.4);
    if ((position.x * 40.0).sin()).abs() > 0.5 {
        ring_color1
    } else {
        ring_color2
    }
}

// Shader para la luna orbitando alrededor del planeta rocoso
pub fn moon_shader(position: Vector3<f32>, time: f32) -> Vector3<f32> {
    let orbit_radius = 0.45;
    let moon_center = Vector3::new(0.5 + orbit_radius * time.cos(), 0.5 + orbit_radius * time.sin(), 0.0);
    let distance = (position - moon_center).magnitude();
    let moon_radius = 0.05;

    if distance > moon_radius {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    Vector3::new(0.8, 0.8, 0.8)
}
