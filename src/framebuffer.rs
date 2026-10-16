pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
    pub depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
            depth: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
        self.depth.fill(f32::INFINITY);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, z: f32, color: u32) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height || !z.is_finite() {
            return;
        }

        let index = y * self.width + x;

        if z < self.depth[index] {
            self.depth[index] = z;
            self.pixels[index] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.pixels[y * self.width + x])
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        let framebuffer = Framebuffer::new(800, 600);

        assert_eq!(framebuffer.pixels.len(), 480000);
        assert_eq!(framebuffer.depth.len(), 480000);
    }

    #[test]
    fn test_pixel() {
        let mut framebuffer = Framebuffer::new(10, 10);

        framebuffer.set_pixel(3, 4, 0.5, rgb(255, 0, 0));

        assert_eq!(framebuffer.get_pixel(3, 4), Some(0xFF0000));
    }

    #[test]
    fn test_depth() {
        let mut framebuffer = Framebuffer::new(10, 10);

        framebuffer.set_pixel(5, 5, 0.8, rgb(255, 0, 0));
        framebuffer.set_pixel(5, 5, 0.9, rgb(0, 255, 0));

        assert_eq!(framebuffer.get_pixel(5, 5), Some(0xFF0000));

        framebuffer.set_pixel(5, 5, 0.2, rgb(0, 0, 255));

        assert_eq!(framebuffer.get_pixel(5, 5), Some(0x0000FF));
    }

    #[test]
    fn test_clear() {
        let mut framebuffer = Framebuffer::new(10, 10);

        framebuffer.set_pixel(2, 2, 0.3, rgb(255, 0, 0));
        framebuffer.clear(rgb(10, 20, 30));

        assert_eq!(framebuffer.get_pixel(2, 2), Some(rgb(10, 20, 30)));

        assert_eq!(framebuffer.depth[22], f32::INFINITY);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut framebuffer = Framebuffer::new(10, 10);

        framebuffer.set_pixel(-1, 5, 0.5, rgb(255, 0, 0));
        framebuffer.set_pixel(20, 20, 0.5, rgb(255, 0, 0));

        assert_eq!(framebuffer.get_pixel(20, 20), None);
    }
}
