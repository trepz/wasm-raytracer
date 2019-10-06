mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        utils::set_panic_hook();
        Canvas { width, height }
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let mut pixels = self.get_pixel_values();
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut pixels),
            self.width,
            self.height,
        )?;
        ctx.put_image_data(&data, 0.0, 0.0)
    }

    pub fn get_pixel_values(&self) -> Vec<u8> {
        let pixels = (0..self.width * self.height)
            .map(|_| vec![255, 0, 0, 255])
            .flatten()
            .collect();
        pixels
    }
}
