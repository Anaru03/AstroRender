mod framebuffer;
mod obj;
mod triangle;

use obj::Model;

fn main() {
    let model =
        Model::load("assets/models/astro_nave.obj").expect("No se pudo cargar astro_nave.obj");

    println!("AstroRender - Lab 4");
    println!("Modelo cargado correctamente");
    println!("Vertices: {}", model.vertices.len());
    println!("Triangulos: {}", model.triangles.len());

    for (i, triangle) in model.triangles.iter().take(3).enumerate() {
        let v0 = model.vertices[triangle[0]];
        let v1 = model.vertices[triangle[1]];
        let v2 = model.vertices[triangle[2]];

        println!("Triangulo {}:", i + 1);
        println!("  Vertice 1: ({}, {}, {})", v0.x, v0.y, v0.z);
        println!("  Vertice 2: ({}, {}, {})", v1.x, v1.y, v1.z);
        println!("  Vertice 3: ({}, {}, {})", v2.x, v2.y, v2.z);
    }
}
