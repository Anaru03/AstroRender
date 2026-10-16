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

fn render(framebuffer: &mut Framebuffer, model: &Model, angle_y: f32, angle_x: f32) -> usize {
    framebuffer.clear(rgb(10, 15, 30));

    render_model(framebuffer, model, angle_y, angle_x)
}

fn main() {
    let model =
        Model::load("assets/models/astro_nave.obj").expect("No se pudo cargar astro_nave.obj");

    println!("AstroRender - Lab 4");
    println!("Vertices: {}", model.vertices.len());
    println!("Triangulos: {}", model.triangles.len());

    let mut window = init();
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut angle_y = 25.0_f32;
    let mut angle_x = -25.0_f32;

    println!("Controles:");
    println!("Flecha izquierda: girar a la izquierda");
    println!("Flecha derecha: girar a la derecha");
    println!("Flecha arriba: inclinar hacia arriba");
    println!("Flecha abajo: inclinar hacia abajo");
    println!("R: reiniciar vista");
    println!("ESC: salir");

    let mut previous_angles = (f32::NAN, f32::NAN);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            angle_y -= 1.5;
        }

        if window.is_key_down(Key::Right) {
            angle_y += 1.5;
        }

        if window.is_key_down(Key::Up) {
            angle_x -= 1.5;
        }

        if window.is_key_down(Key::Down) {
            angle_x += 1.5;
        }

        if window.is_key_down(Key::R) {
            angle_y = 25.0;
            angle_x = -25.0;
        }

        angle_x = angle_x.clamp(-89.0, 89.0);

        if previous_angles != (angle_y, angle_x) {
            let rendered = render(&mut framebuffer, &model, angle_y, angle_x);

            previous_angles = (angle_y, angle_x);

            println!(
                "Triangulos procesados: {} | Rotacion Y: {:.1} | Rotacion X: {:.1}",
                rendered, angle_y, angle_x
            );
        }

        window
            .update_with_buffer(&framebuffer.pixels, framebuffer.width, framebuffer.height)
            .expect("No se pudo actualizar la ventana");
    }
}
