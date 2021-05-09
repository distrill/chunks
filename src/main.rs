use tetra::graphics::{self, Color, DrawParams, Camera};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, State, ContextBuilder};

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
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let map = Map::new(ctx)?;

        let mut camera = Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        camera.position.x = 0.0 as f32;
        camera.position.y = 0.0 as f32;
        camera.scale = Vec2::new(2.0, 2.0);

        let player = Player::new(ctx)?;

        Ok(GameState { map, camera, player })
    }

    fn handle_player_move(&mut self, ctx: &mut Context, direction: Direction) -> tetra::Result {
        let old_pos = match direction {
            Direction::Up => self.camera.position.y,
            Direction::Down => self.camera.position.y,
            Direction::Left => self.camera.position.x,
            Direction::Right => self.camera.position.x,
        };
        let new_pos = match direction {
            Direction::Up => old_pos - MOVEMENT_SPEED,
            Direction::Down => old_pos + MOVEMENT_SPEED,
            Direction::Left => old_pos - MOVEMENT_SPEED,
            Direction::Right => old_pos + MOVEMENT_SPEED,
        };
        
        match direction {
            Direction::Up => {
                self.camera.position.y = new_pos;
                self.player.position.y = new_pos;
            }
            Direction::Down => {
                self.camera.position.y = new_pos;
                self.player.position.y = new_pos;
            }
            Direction::Left => {
                self.camera.position.x = new_pos;
                self.player.position.x = new_pos;
            }
            Direction::Right => {
                self.camera.position.x = new_pos;
                self.player.position.x = new_pos;
            }
        }

        if (old_pos / CHUNK_SIZE) as i32 != (new_pos / CHUNK_SIZE) as i32 {
            self.map.move_map(ctx, &direction)?;
        }

        Ok(())
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
        if input::is_key_down(ctx, Key::W) {
            self.handle_player_move(ctx, Direction::Up)?;
        }

        if input::is_key_down(ctx, Key::S) {
            self.handle_player_move(ctx, Direction::Down)?;
        }

        if input::is_key_down(ctx, Key::A) {
            self.handle_player_move(ctx, Direction::Left)?;
        }

        if input::is_key_down(ctx, Key::D) {
            self.handle_player_move(ctx, Direction::Right)?;
        }


        if input::is_key_pressed(ctx, Key::W) {
            self.player.animation.set_state(PlayerState::Running);
        }

        if input::is_key_pressed(ctx, Key::S) {
            self.player.animation.set_state(PlayerState::Running);
        }

        if input::is_key_pressed(ctx, Key::A) {
            self.player.animation.set_state(PlayerState::Running);
        }

        if input::is_key_pressed(ctx, Key::D) {
            self.player.animation.set_state(PlayerState::Running);
        }

        if input::is_key_released(ctx, Key::W) {
            self.player.animation.set_state(PlayerState::Idle);
        }

        if input::is_key_released(ctx, Key::S) {
            self.player.animation.set_state(PlayerState::Idle);
        }

        if input::is_key_released(ctx, Key::A) {
            self.player.animation.set_state(PlayerState::Idle);
        }

        if input::is_key_released(ctx, Key::D) {
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
    ContextBuilder::new("bc", 1280, 720)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
