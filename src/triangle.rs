use crate::framebuffer::Framebuffer;

#[derive(Clone, Copy, Debug)]
pub struct ScreenVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ScreenVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

fn edge(a: ScreenVertex, b: ScreenVertex, p: ScreenVertex) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

pub fn draw_triangle(
    framebuffer: &mut Framebuffer,
    v0: ScreenVertex,
    v1: ScreenVertex,
    v2: ScreenVertex,
    color: u32,
) {
    if framebuffer.width == 0 || framebuffer.height == 0 {
        return;
    }

    let area = edge(v0, v1, v2);

    if !area.is_finite() || area.abs() < 1e-6 {
        return;
    }

    let min_x = v0.x.min(v1.x).min(v2.x).floor().max(0.0) as i32;
    let max_x =
        v0.x.max(v1.x)
            .max(v2.x)
            .ceil()
            .min((framebuffer.width - 1) as f32) as i32;

    let min_y = v0.y.min(v1.y).min(v2.y).floor().max(0.0) as i32;
    let max_y =
        v0.y.max(v1.y)
            .max(v2.y)
            .ceil()
            .min((framebuffer.height - 1) as f32) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = ScreenVertex::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            let w0 = edge(v1, v2, p) / area;
            let w1 = edge(v2, v0, p) / area;
            let w2 = edge(v0, v1, p) / area;

            if w0 >= -1e-6 && w1 >= -1e-6 && w2 >= -1e-6 {
                let depth = w0 * v0.z + w1 * v1.z + w2 * v2.z;

                framebuffer.set_pixel(x, y, depth, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::rgb;

    #[test]
    fn test_triangle_fills_pixels() {
        let mut framebuffer = Framebuffer::new(100, 100);
        framebuffer.clear(rgb(0, 0, 0));

        draw_triangle(
            &mut framebuffer,
            ScreenVertex::new(10.0, 10.0, 0.5),
            ScreenVertex::new(80.0, 10.0, 0.5),
            ScreenVertex::new(40.0, 80.0, 0.5),
            rgb(255, 200, 0),
        );

        assert_eq!(framebuffer.get_pixel(40, 40), Some(rgb(255, 200, 0)));
    }

    #[test]
    fn test_triangle_outside_screen() {
        let mut framebuffer = Framebuffer::new(100, 100);
        framebuffer.clear(rgb(0, 0, 0));

        draw_triangle(
            &mut framebuffer,
            ScreenVertex::new(-200.0, -200.0, 0.5),
            ScreenVertex::new(-100.0, -200.0, 0.5),
            ScreenVertex::new(-150.0, -100.0, 0.5),
            rgb(255, 0, 0),
        );

        assert_eq!(framebuffer.get_pixel(50, 50), Some(0));
    }

    #[test]
    fn test_degenerate_triangle() {
        let mut framebuffer = Framebuffer::new(100, 100);

        draw_triangle(
            &mut framebuffer,
            ScreenVertex::new(10.0, 10.0, 0.5),
            ScreenVertex::new(20.0, 20.0, 0.5),
            ScreenVertex::new(30.0, 30.0, 0.5),
            rgb(255, 0, 0),
        );

        assert_eq!(framebuffer.get_pixel(20, 20), Some(0));
    }

    #[test]
    fn test_triangle_depth() {
        let mut framebuffer = Framebuffer::new(100, 100);

        let a = ScreenVertex::new(10.0, 10.0, 0.0);
        let b = ScreenVertex::new(80.0, 10.0, 0.0);
        let c = ScreenVertex::new(40.0, 80.0, 0.0);

        draw_triangle(
            &mut framebuffer,
            ScreenVertex::new(a.x, a.y, 0.8),
            ScreenVertex::new(b.x, b.y, 0.8),
            ScreenVertex::new(c.x, c.y, 0.8),
            rgb(255, 0, 0),
        );

        draw_triangle(
            &mut framebuffer,
            ScreenVertex::new(a.x, a.y, 0.2),
            ScreenVertex::new(b.x, b.y, 0.2),
            ScreenVertex::new(c.x, c.y, 0.2),
            rgb(0, 255, 0),
        );

        assert_eq!(framebuffer.get_pixel(40, 40), Some(rgb(0, 255, 0)));
    }
}
