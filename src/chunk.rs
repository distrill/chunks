use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::math::Vec2;
use tetra::Context;

use crate::constants::*;

pub struct Chunk {
    x: i32,
    y: i32,
    mesh: Mesh,
    label: Text,
}

impl Chunk {
    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        let position = 
            Vec2::new(self.x as f32 * CHUNK_SIZE, self.y as f32 * CHUNK_SIZE);
        self.mesh.draw(
            ctx,
            DrawParams::new()
                .color(DEBUG_FONT_COLOR)
                .position(position),
        );
        self.label.draw(
            ctx,
            DrawParams::new()
                .color(DEBUG_FONT_COLOR)
                .position(position),
        );

        Ok(())
    }

    pub fn new(ctx: &mut Context, x: i32, y: i32) -> tetra::Result<Chunk> {
        let mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(1.0),
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: CHUNK_SIZE,
                height: CHUNK_SIZE,
            },
        )?;
        let label = Text::new(
            format!("({}, {})", x, y),
            Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 12.0)?,
        );
        Ok(Chunk { x, y, mesh, label })
    }
}
