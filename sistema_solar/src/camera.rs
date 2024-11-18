use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub speed: f32,
}

impl Camera {
    // Constructor para la cámara
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 5.0), // Posición inicial de la cámara
            speed: 0.05, // Velocidad de movimiento de la cámara
        }
    }

    // Método para mover la cámara
    pub fn move_camera(&mut self, direction: &str) {
        match direction {
            "left" => self.position.x -= self.speed,
            "right" => self.position.x += self.speed,
            "up" => self.position.y += self.speed,
            "down" => self.position.y -= self.speed,
            "forward" => self.position.z -= self.speed,
            "backward" => self.position.z += self.speed,
            _ => {},
        }
    }

    // Método para obtener la matriz de vista
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::new(self.position.x, self.position.y, self.position.z),
            &Point3::new(0.0, 0.0, 0.0), // Apuntando al origen
            &Vector3::y_axis(),
        )
    }
}
