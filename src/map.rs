use std::collections::HashMap;

use tetra::{Context};

use crate::constants::*;
use crate::chunk::{Chunk};

fn get_initial_draw_magnitude(pixel_magnitude: f32) -> f32 {
    ((pixel_magnitude / CHUNK_SIZE) / 2 as f32) + 2 as f32
}

pub struct VisibleMap {
    pub minx: i32,
    pub miny: i32,
    pub maxx: i32,
    pub maxy: i32,
}

impl VisibleMap {
    fn new() -> VisibleMap {
        VisibleMap {
            minx: 0  - get_initial_draw_magnitude(WINDOW_WIDTH) as i32,
            maxx: get_initial_draw_magnitude(WINDOW_WIDTH) as i32,
            miny: 0 - get_initial_draw_magnitude(WINDOW_HEIGHT) as i32,
            maxy: get_initial_draw_magnitude(WINDOW_HEIGHT) as i32,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Map {
    pub chunks: HashMap<i32, HashMap<i32, Chunk>>,
    pub drawn: VisibleMap,
}

impl Map {
    pub fn new(ctx: &mut Context) -> tetra::Result<Map> {
        // let mut chunks = Vec::with_capacity(height);
        let mut chunks = HashMap::new();
        let drawn = VisibleMap::new();

        for i in drawn.minx - 55..drawn.maxx + 55 {
            let mut col = HashMap::new();
            for j in drawn.miny - 55.. drawn.maxy + 55 {
                let chunk = Chunk::new(ctx, i as i32, j as i32)?;
                col.insert(j, chunk);
            }
            chunks.insert(i, col);
        }

        Ok(Map { chunks, drawn })
    }

    /// We draw whatever is on the screen, plus 2 chunks in all directions
    /// We load whatever is on the screen, plus 50 chunks in all directions
    /// When we move the camera, we update what is draw and loaded
    pub fn move_map(&mut self, ctx: &mut Context, direction: &Direction) -> tetra::Result {
        match direction {
            Direction::Up => {
                self.drawn.miny -= 1;
                self.drawn.maxy -= 1;
                for i in (self.drawn.minx - 50)..(self.drawn.maxx + 50) {
                    let j = self.drawn.miny - 50;
                    let col = self.chunks.entry(i).or_insert(HashMap::new());
                    col.entry(j).or_insert(Chunk::new(ctx, i, j)?);
                }
            }
            Direction::Down => {
                self.drawn.miny += 1;
                self.drawn.maxy += 1;
                for i in (self.drawn.minx - 50)..(self.drawn.maxx + 50) {
                    let j = self.drawn.maxy + 50;
                    let col = self.chunks.entry(i).or_insert(HashMap::new());
                    col.entry(j).or_insert(Chunk::new(ctx, i, j)?);
                }
            }
            Direction::Left => {
                self.drawn.minx -= 1;
                self.drawn.maxx -= 1;
                for j in (self.drawn.miny - 50)..(self.drawn.maxy + 50) {
                    let i = self.drawn.minx - 50;
                    let col = self.chunks.entry(i).or_insert(HashMap::new());
                    col.entry(j).or_insert(Chunk::new(ctx, i, j)?);
                }
            }
            Direction::Right => {
                self.drawn.minx += 1;
                self.drawn.maxx += 1;
                for j in (self.drawn.miny - 50)..(self.drawn.maxy + 50) {
                    let i = self.drawn.maxx + 50;
                    let col = self.chunks.entry(i).or_insert(HashMap::new());
                    col.entry(j).or_insert(Chunk::new(ctx, i, j)?);
                }
            }
        }

        Ok(())
    }
}
