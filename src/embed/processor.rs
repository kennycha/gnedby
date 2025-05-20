use anyhow::Result;
use ndarray::Array4;

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_image(&self, image_data: &[u8]) -> Result<Array4<f32>> {
        let img = image::load_from_memory(image_data)?;
        let resized = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);
        let rgb = resized.to_rgb8();
        let mut tensor = Array4::zeros((1, 3, 224, 224));

        for c in 0..3 {
            for y in 0..224 {
                for x in 0..224 {
                    let pixel = rgb.get_pixel(x as u32, y as u32);
                    tensor[[0, c, y as usize, x as usize]] = pixel[c] as f32 / 255.0;
                }
            }
        }

        Ok(tensor)
    }
}
