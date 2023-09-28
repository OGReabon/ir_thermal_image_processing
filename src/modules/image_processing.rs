struct IrImage {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

struct ThermalImage {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl ThermalImage {
    fn resize(&self, new_width: usize, new_height: usize) -> ThermalImage {
        let mut resized_pixels = Vec::with_capacity(new_width * new_height * 3);

        for y in 0..new_height {
            for x in 0..new_width {
                // Map the new pixel location to the original image
                let src_x = x as f32 * (self.width as f32 / new_width as f32);
                let src_y = y as f32 * (self.height as f32 / new_height as f32);

                // Fill the coordinates of the four surrounding pixels
                let x0 = src_x.floor() as usize;
                let x1 = x0.saturating_add(1).min(self.width - 1);
                let y0 = src_y.floor() as usize;
                let y1 = y0.saturating_add(1).min(self.height - 1);

                // Calculate the weights for each surrounding pixel
                let dx = src_x - x0 as f32;
                let dy = src_y - y0 as f32;

                // Bilinear interpolation for each surrounding pixel
                for channel in 0..3 {
                    let index = [x, y](y * self.width + x) * 3 + channel;
                    let value = (1.0) * (1.0 - dy) * self.pixels[index(x0, y0)] as f32
                        + dx * (1.0 - dy) * self.pixels[index(x1, y0)] as f32
                        + (1.0 - dx) * dy * self.pixels[index(x0, y1)] as f32
                        + dx * dy * self.pixels[index(x1, y1)] as f32;

                    pixels.push(value as u8);
                }
            }
        }

        ThermalImage {
            pixels: resized_pixels,
            width: new_width,
            height: new_height,
        }
    }
}
