mod framebuffer;
mod obj;
mod renderer;
mod triangle;

use framebuffer::{Framebuffer, rgb};
use obj::Model;
use renderer::render_model;

fn main() {
    let model =
        Model::load("assets/models/astro_nave.obj").expect("No se pudo cargar astro_nave.obj");

    println!("AstroRender - Lab 4");
    println!("Vertices: {}", model.vertices.len());
    println!("Triangulos: {}", model.triangles.len());

    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.clear(rgb(10, 15, 30));

    let rendered = render_model(&mut framebuffer, &model);

    println!("Triangulos procesados: {}", rendered);
    println!("Renderizado completado");
}
