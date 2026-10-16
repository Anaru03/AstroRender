mod framebuffer;
mod obj;
mod renderer;
mod triangle;

use framebuffer::{Framebuffer, rgb};
use minifb::{Key, Window, WindowOptions};
use obj::Model;
use renderer::render_model;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn init() -> Window {
    let mut window = Window::new(
        "AstroRender | Lab 4 - Carga de Modelos",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(60);

    window
}

fn render(framebuffer: &mut Framebuffer, model: &Model) -> usize {
    framebuffer.clear(rgb(10, 15, 30));
    render_model(framebuffer, model)
}

fn main() {
    let model =
        Model::load("assets/models/astro_nave.obj").expect("No se pudo cargar astro_nave.obj");

    println!("AstroRender - Lab 4");
    println!("Vertices: {}", model.vertices.len());
    println!("Triangulos: {}", model.triangles.len());

    let mut window = init();
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let rendered = render(&mut framebuffer, &model);

    println!("Triangulos procesados: {}", rendered);
    println!("Renderizado completado");
    println!("Presiona ESC para salir");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&framebuffer.pixels, framebuffer.width, framebuffer.height)
            .expect("No se pudo actualizar la ventana");
    }
}
