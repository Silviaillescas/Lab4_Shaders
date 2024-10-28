use nalgebra::Vector3;

// Shader para la estrella con material emisivo avanzado
pub fn emissive_star_shader(position: Vector3<f32>) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0); // Centro de la estrella
    let distance = (position - center).magnitude();
    let star_radius = 0.2;

    if distance > star_radius {
        return Vector3::new(0.0, 0.0, 0.0); // Fondo negro fuera de la estrella
    }

    // Crear un efecto de emisión de luz
    let intensity = (1.0 - distance / star_radius).powf(3.0); // Ajuste de intensidad de brillo
    let star_color = Vector3::new(1.0, 0.9, 0.6); // Color amarillo claro para la estrella

    star_color * intensity
}

// Shader para el planeta rocoso con efecto de iluminación difusa
pub fn rocky_planet_shader(position: Vector3<f32>, time: f32) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0); // Centro del planeta
    let planet_radius = 0.3; // Radio del planeta

    let distance = (position - center).magnitude();
    if distance > planet_radius {
        return Vector3::new(0.0, 0.0, 0.0); // Fondo negro fuera del planeta
    }

    let sphere_normal = (position - center).normalize();
    let light_dir = Vector3::new(0.4, -0.3, 0.7).normalize();
    let diff_intensity = sphere_normal.dot(&light_dir).max(0.5);

    let noise = (simple_noise(position.x * 1.5, position.y * 1.5) * 0.5 + 0.5).powf(1.5);
    let ocean_color = Vector3::new(0.0, 0.3, 0.6);
    let land_color = Vector3::new(0.5, 0.7, 0.4);
    let color = land_color * noise + ocean_color * (1.0 - noise);

    let cloud_layer = simple_noise(position.x + time * 0.05, position.y + time * 0.05) * 0.3;
    let atmosphere_color = Vector3::new(0.9, 0.9, 1.0) * cloud_layer;

    let halo_strength = (1.0 - (distance / planet_radius).powf(3.0)).max(0.0);
    let halo_color = atmosphere_color * halo_strength;

    color * diff_intensity + halo_color
}

// Shader para un planeta gaseoso con bandas de color
pub fn gas_giant_shader(position: Vector3<f32>, time: f32) -> Vector3<f32> {
    let center = Vector3::new(0.5, 0.5, 0.0);
    let distance = (position - center).magnitude();
    let planet_radius = 0.35;

    if distance > planet_radius {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let band_pattern = ((position.y * 10.0 + time * 0.2).sin() * 0.5 + 0.5).abs();
    let color1 = Vector3::new(0.7, 0.4, 0.2);
    let color2 = Vector3::new(0.3, 0.6, 0.7);

    color1 * band_pattern + color2 * (1.0 - band_pattern)
}

// Shader para los anillos alrededor del planeta gaseoso
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

// Shader para una luna orbitando alrededor del planeta rocoso
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

// Shader para un asteroide adicional en el sistema

pub fn asteroid_shader(position: Vector3<f32>) -> Vector3<f32> {
    // Crear un círculo en el centro
    let center = Vector3::new(0.5, 0.5, 0.0);
    let distance = (position - center).magnitude();
    let asteroid_radius = 0.3; // Aumentamos el radio para que sea claramente visible

    if distance <= asteroid_radius {
        return Vector3::new(1.0, 0.0, 0.0); // Color rojo para que sea fácilmente visible
    }

    // Fondo gris para identificar si se está ejecutando este shader
    Vector3::new(0.5, 0.5, 0.5)
}

// Función de ruido simple para crear variación de textura
fn simple_noise(x: f32, y: f32) -> f32 {
    ((x * 10.0).sin() + (y * 10.0).cos()).abs() * 0.5
}
