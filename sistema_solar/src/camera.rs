use nalgebra::Vector3;

pub struct Camera {
    pub position: Vector3<f32>,
    pub fov: f32, // Campo de visión de la cámara
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            fov: 45.0, // FOV inicial
        }
    }

    // Mover la cámara en la dirección especificada
    pub fn move_camera(&mut self, direction: &str) {
        match direction {
            "forward" => self.position.z -= 0.5,
            "backward" => self.position.z += 0.5,
            "left" => self.position.x -= 0.5,
            "right" => self.position.x += 0.5,
            _ => {}
        }
    }

    // Animar la cámara hacia una posición objetivo con interpolación suave
    pub fn animate_to(&mut self, target: (f32, f32, f32), speed: f32, progress: f32) {
        let target_vec = Vector3::new(target.0, target.1, target.2);
        let direction = (target_vec - self.position) * progress;

        // Actualizar posición usando interpolación suave
        self.position += direction * speed;

        // Cambiar el FOV para un efecto de zoom más sutil
        self.fov = 45.0 + 15.0 * (1.0 - progress).powi(2); // Cambiar a 15.0 para un zoom más sutil
    }

    // Verificar si la cámara ha llegado al destino
    pub fn has_reached(&self, target: (f32, f32, f32)) -> bool {
        let target_vec = Vector3::new(target.0, target.1, target.2);
        (target_vec - self.position).magnitude() < 0.1
    }

    // Obtener la matriz de vista para el renderizado
    pub fn get_view_matrix(&self) -> nalgebra::Matrix4<f32> {
        nalgebra::Matrix4::new_translation(&-self.position)
    }

    // Obtener la matriz de proyección con FOV dinámico
    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> nalgebra::Matrix4<f32> {
        nalgebra::Matrix4::new_perspective(aspect_ratio, self.fov.to_radians(), 0.1, 100.0)
    }
}
