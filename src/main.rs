use tetra::graphics::{self, Color, DrawParams, Camera};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, State, ContextBuilder};

#[macro_use]
extern crate log;

mod constants;
mod map;
mod chunk;
mod player;

use map::{Map, Direction};
use player::{Player, PlayerState};
use constants::*;


struct GameState {
    map: Map,
    camera: Camera,
    player: Player,
    velocity: Vec2<f32>,
    position: Vec2<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let map = Map::new(ctx)?;

        let velocity = Vec2::new(0.0, 0.0);
        let position = Vec2::new(0.0, 0.0);

        let mut camera = Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        camera.scale = Vec2::new(2.0, 2.0);
        camera.position = position;

        let player = Player::new(ctx, position)?;

        Ok(GameState { map, camera, player, velocity, position })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, GLOBAL_BACKGROUND_COLOR);

        // look through the camera
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        self.player.animation.advance(ctx);
        self.player.animation.draw(
            ctx,
            DrawParams::new()
                .position(self.player.position),
        );

        // draw each chunk
        for i in self.map.drawn.minx..self.map.drawn.maxx {
            for j in self.map.drawn.miny..self.map.drawn.maxy {
                if let Some(col) = self.map.chunks.get_mut(&i) {
                    if let Some(chunk) = col.get_mut(&j) {
                        chunk.draw(ctx)?;
                    } else {
                        println!("UNABLE TO LOAD CHUNK ROW {}", j);
                    }
                } else {
                    println!("UNABLE TO LOAD CHUNK COL {}", i);
                }
            }
        }


        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mut is_vert_moving_key_down = false;
        let mut is_horiz_moving_key_down = false;
        if input::is_key_down(ctx, Key::W) {
            self.velocity.y = (self.velocity.y - 0.5).max(-5.0);
            is_vert_moving_key_down = true;
        }

        if input::is_key_down(ctx, Key::S) {
            self.velocity.y = (self.velocity.y + 0.5).min(5.0);
            is_vert_moving_key_down = true;
        }

        if input::is_key_down(ctx, Key::A) {
            self.velocity.x = (self.velocity.x - 0.5).max(-5.0);
            is_horiz_moving_key_down = true;
        }

        if input::is_key_down(ctx, Key::D) {
            self.velocity.x = (self.velocity.x + 0.5).min(5.0);
            is_horiz_moving_key_down = true;
        }

        if !is_vert_moving_key_down {
            self.velocity.y -= self.velocity.y.abs().min(0.5) * self.velocity.y.signum();
        }

        if !is_horiz_moving_key_down {
            self.velocity.x -= self.velocity.x.abs().min(0.5) * self.velocity.x.signum();
        }

        let old_x = self.position.x;
        let old_y = self.position.y;
        
        let new_x = old_x + self.velocity.x;
        let new_y = old_y + self.velocity.y;


        
        if ((old_y / CHUNK_SIZE) as i32) < ((new_y / CHUNK_SIZE) as i32) {
            self.map.move_map(ctx, &Direction::Down)?;
        } else if ((old_y / CHUNK_SIZE) as i32) > ((new_y / CHUNK_SIZE) as i32) {
            self.map.move_map(ctx, &Direction::Up)?;
        }

        if ((old_x / CHUNK_SIZE) as i32) < ((new_x / CHUNK_SIZE) as i32) {
            self.map.move_map(ctx, &Direction::Right)?;
        } else if ((old_x / CHUNK_SIZE) as i32) > ((new_x / CHUNK_SIZE) as i32) {
            self.map.move_map(ctx, &Direction::Left)?;
        }

        let new_pos = Vec2::new(new_x, new_y);
        self.position = new_pos;
        self.camera.position = new_pos;
        self.player.position = new_pos;

        if self.velocity.x.abs() > 0.0 || self.velocity.y.abs() > 0.0 {
            self.player.animation.set_state(PlayerState::Running);
        } else {
            self.player.animation.set_state(PlayerState::Idle);
        }

        if input::is_mouse_scrolled_up(ctx) {
            let newx = self.camera.scale.x + ZOOM_SPEED;
            let newy = self.camera.scale.y + ZOOM_SPEED;
            if newx <= 2.0 {
                self.camera.scale.x = newx;
            }
            if newy <= 2.0 {
                self.camera.scale.y = newy;
            }
        }

        if input::is_mouse_scrolled_down(ctx) {
            let newx = self.camera.scale.x - ZOOM_SPEED;
            let newy = self.camera.scale.y - ZOOM_SPEED;
            if newx >= 1.0 {
                self.camera.scale.x = newx;
            }
            if newy >= 1.0 {
                self.camera.scale.y = newy;
            }
        }
   
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.camera.update();

        Ok(())
    }
}

fn main() -> tetra::Result {
    env_logger::init();

    info!("starting up");

    ContextBuilder::new("bc", 1280, 720)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
