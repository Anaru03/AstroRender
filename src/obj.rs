use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[usize; 3]>,
}

impl Model {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("No se pudo abrir el OBJ: {e}"))?;

        let reader = BufReader::new(file);
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| e.to_string())?;
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("v") => {
                    let coords: Vec<f32> = parts
                        .take(3)
                        .map(|value| value.parse::<f32>())
                        .collect::<Result<_, _>>()
                        .map_err(|e| format!("Linea {}: {e}", line_number + 1))?;

                    if coords.len() != 3 {
                        return Err(format!("Vertice incompleto en linea {}", line_number + 1));
                    }

                    vertices.push(Vertex {
                        x: coords[0],
                        y: coords[1],
                        z: coords[2],
                    });
                }

                Some("f") => {
                    let indices: Vec<usize> = parts
                        .map(|value| {
                            let index = value
                                .split('/')
                                .next()
                                .ok_or("Indice vacio")?
                                .parse::<usize>()
                                .map_err(|_| "Indice invalido")?;

                            if index == 0 || index > vertices.len() {
                                return Err("Indice fuera de rango");
                            }

                            Ok(index - 1)
                        })
                        .collect::<Result<_, _>>()
                        .map_err(|e| format!("Linea {}: {e}", line_number + 1))?;

                    if indices.len() != 3 {
                        return Err(format!("Cara no triangular en linea {}", line_number + 1));
                    }

                    triangles.push([indices[0], indices[1], indices[2]]);
                }

                _ => {}
            }
        }

        Ok(Self {
            vertices,
            triangles,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_spaceship() {
        let model = Model::load("assets/models/astro_nave.obj").expect("No se pudo cargar la nave");

        assert_eq!(model.vertices.len(), 659);
        assert_eq!(model.triangles.len(), 1310);
    }

    #[test]
    fn test_triangle_indices() {
        let model = Model::load("assets/models/astro_nave.obj").expect("No se pudo cargar la nave");

        for triangle in &model.triangles {
            for &index in triangle {
                assert!(index < model.vertices.len());
            }
        }
    }
}
