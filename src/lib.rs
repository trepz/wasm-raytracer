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
pub fn render(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let mut pixels = get_pixel_values(width, height);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_pixel_values(width: u32, height: u32) -> Vec<u8> {
    let pixels = (0..width * height)
        .map(|_| vec![255, 0, 0, 255])
        .flatten()
        .collect();
    pixels
}
