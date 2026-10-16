use crate::framebuffer::{Framebuffer, rgb};
use crate::obj::Model;
use crate::triangle::{ScreenVertex, draw_triangle};

pub fn render_model(
    framebuffer: &mut Framebuffer,
    model: &Model,
    angle_y_degrees: f32,
    angle_x_degrees: f32,
) -> usize {
    if model.vertices.is_empty() || framebuffer.width == 0 || framebuffer.height == 0 {
        return 0;
    }

    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];

    for vertex in &model.vertices {
        let coordinates = [vertex.x, vertex.y, vertex.z];

        for i in 0..3 {
            min[i] = min[i].min(coordinates[i]);
            max[i] = max[i].max(coordinates[i]);
        }
    }

    let center = [
        (min[0] + max[0]) * 0.5,
        (min[1] + max[1]) * 0.5,
        (min[2] + max[2]) * 0.5,
    ];

    let angle_y = angle_y_degrees.to_radians();
    let angle_x = angle_x_degrees.to_radians();

    let (sy, cy) = angle_y.sin_cos();
    let (sx, cx) = angle_x.sin_cos();

    let mut transformed = Vec::with_capacity(model.vertices.len());

    for vertex in &model.vertices {
        let x = vertex.x - center[0];
        let y = vertex.y - center[1];
        let z = vertex.z - center[2];

        let rx = x * cy + z * sy;
        let rz = -x * sy + z * cy;

        let ry = y * cx - rz * sx;
        let rz = y * sx + rz * cx;

        transformed.push([rx, ry, rz]);
    }

    let mut projected_min = [f32::INFINITY; 2];
    let mut projected_max = [f32::NEG_INFINITY; 2];

    for vertex in &transformed {
        projected_min[0] = projected_min[0].min(vertex[0]);
        projected_min[1] = projected_min[1].min(vertex[1]);

        projected_max[0] = projected_max[0].max(vertex[0]);
        projected_max[1] = projected_max[1].max(vertex[1]);
    }

    let projected_width = projected_max[0] - projected_min[0];
    let projected_height = projected_max[1] - projected_min[1];

    let scale_x = framebuffer.width as f32 * 0.72 / projected_width.max(0.001);

    let scale_y = framebuffer.height as f32 * 0.72 / projected_height.max(0.001);

    let scale = scale_x.min(scale_y);

    let projected_center = [
        (projected_min[0] + projected_max[0]) * 0.5,
        (projected_min[1] + projected_max[1]) * 0.5,
    ];

    let screen_vertices: Vec<ScreenVertex> = transformed
        .iter()
        .map(|vertex| {
            ScreenVertex::new(
                framebuffer.width as f32 * 0.5 + (vertex[0] - projected_center[0]) * scale,
                framebuffer.height as f32 * 0.5 - (vertex[1] - projected_center[1]) * scale,
                -vertex[2],
            )
        })
        .collect();

    let light = [0.35_f32, 0.65, 0.68];

    let light_length = (light[0] * light[0] + light[1] * light[1] + light[2] * light[2]).sqrt();

    let mut rendered = 0;

    for triangle in &model.triangles {
        let [i0, i1, i2] = *triangle;

        let a = transformed[i0];
        let b = transformed[i1];
        let c = transformed[i2];

        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];

        let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];

        let normal = [
            ab[1] * ac[2] - ab[2] * ac[1],
            ab[2] * ac[0] - ab[0] * ac[2],
            ab[0] * ac[1] - ab[1] * ac[0],
        ];

        let normal_length =
            (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();

        if normal_length < 1e-8 {
            continue;
        }

        let diffuse = (normal[0] * light[0] + normal[1] * light[1] + normal[2] * light[2])
            / (normal_length * light_length);

        let intensity = 0.45 + 0.55 * diffuse.abs();

        let color = rgb(
            (220.0 * intensity) as u8,
            (185.0 * intensity) as u8,
            (95.0 * intensity) as u8,
        );

        draw_triangle(
            framebuffer,
            screen_vertices[i0],
            screen_vertices[i1],
            screen_vertices[i2],
            color,
        );

        rendered += 1;
    }

    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_spaceship() {
        let model = Model::load("assets/models/astro_nave.obj").unwrap();

        let mut framebuffer = Framebuffer::new(800, 600);
        let background = rgb(10, 15, 30);

        framebuffer.clear(background);

        let count = render_model(&mut framebuffer, &model, 25.0, -25.0);

        assert!(count > 0);
        assert!(count <= model.triangles.len());

        assert!(framebuffer.pixels.iter().any(|&pixel| pixel != background));
    }

    #[test]
    fn test_model_centered_and_scaled() {
        let model = Model::load("assets/models/astro_nave.obj").unwrap();

        let mut framebuffer = Framebuffer::new(800, 600);

        let background = rgb(10, 15, 30);
        framebuffer.clear(background);

        render_model(&mut framebuffer, &model, 25.0, -25.0);

        let mut min_x = framebuffer.width;
        let mut max_x = 0;
        let mut min_y = framebuffer.height;
        let mut max_y = 0;
        let mut found = false;

        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                if framebuffer.pixels[y * framebuffer.width + x] != background {
                    found = true;
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                }
            }
        }

        assert!(found, "El modelo no produjo pixeles visibles");

        let center_x = (min_x + max_x) as f32 * 0.5;
        let center_y = (min_y + max_y) as f32 * 0.5;

        let model_width = max_x - min_x + 1;
        let model_height = max_y - min_y + 1;

        assert!((center_x - 400.0).abs() < 40.0);
        assert!((center_y - 300.0).abs() < 40.0);

        assert!(model_width > 160);
        assert!(model_height > 120);

        assert!(model_width < 800);
        assert!(model_height < 600);
    }

    #[test]
    fn test_rotated_spaceship() {
        let model = Model::load("assets/models/astro_nave.obj").unwrap();

        let mut framebuffer = Framebuffer::new(800, 600);

        let background = rgb(10, 15, 30);
        framebuffer.clear(background);

        let count = render_model(&mut framebuffer, &model, 90.0, -20.0);

        assert!(count > 0);

        assert!(framebuffer.pixels.iter().any(|&pixel| pixel != background));
    }
}
