use nalgebra::{Point3, Vector3};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Spaceship {
    pub vertices: Vec<Point3<f32>>,
}

impl Spaceship {
    pub fn load_from_obj(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("No se pudo abrir el archivo .obj: {}", e))?;
        let reader = BufReader::new(file);
        let mut vertices = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error al leer la línea del archivo: {}", e))?;
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let (Some(x), Some(y), Some(z)) = (parts.get(1), parts.get(2), parts.get(3)) {
                    let x: f32 = x.parse().map_err(|e| format!("Error al parsear x: {}", e))?;
                    let y: f32 = y.parse().map_err(|e| format!("Error al parsear y: {}", e))?;
                    let z: f32 = z.parse().map_err(|e| format!("Error al parsear z: {}", e))?;
                    vertices.push(Point3::new(x, y, z));
                }
            }
        }

        Ok(Spaceship { vertices })
    }
}
