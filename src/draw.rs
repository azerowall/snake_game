use graphics::{
    types::Color,
    context::Context,
};
use opengl_graphics::GlGraphics;

const BLOCK_SIZE: f64 = 20.0;

fn to_scalar_coord(point: u32) -> f64 {
    point as f64 * BLOCK_SIZE
}

pub fn draw_block(x: u32, y: u32, color: Color, context: &Context, gl: &mut GlGraphics) {
    graphics::rectangle(
        color,
        [to_scalar_coord(x), to_scalar_coord(y), BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        gl);
}

pub fn draw_rectangle(x: u32, y: u32, width: u32, height: u32, color: Color, context: &Context, gl: &mut GlGraphics) {
    graphics::rectangle(
        color,
        [to_scalar_coord(x), to_scalar_coord(y), width as f64 * BLOCK_SIZE, height as f64 * BLOCK_SIZE],
        context.transform,
        gl
    );
}