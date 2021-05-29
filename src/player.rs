use std::time::Duration;

use tetra::graphics::{Texture, Rectangle, DrawParams};
use tetra::graphics::animation::Animation;
use tetra::math::{Vec2};
use tetra::{Context};

#[derive(PartialEq, Debug)]
pub enum PlayerState {
    Idle,
    Running,
}

pub struct PlayerAnimation {
    state: PlayerState,
    idle: Animation,
    running: Animation,
}

impl PlayerAnimation {
    fn new(ctx: &mut Context) -> tetra::Result<PlayerAnimation> {
        let texture = Texture::new(ctx, "./resources/tiles.png")?;

        Ok(PlayerAnimation {
            state: PlayerState::Idle,
            idle: Animation::new(
                texture.clone(),
                Rectangle::row(0.0, 256.0, 16.0, 16.0).take(8).collect(),
                Duration::from_secs_f64(0.1),
            ),
            running: Animation::new(
                texture,
                Rectangle::row(0.0, 272.0, 16.0, 16.0).take(8).collect(),
                Duration::from_secs_f64(0.1),
            ),
        })
    }

    pub fn draw<P>(&self, ctx: &mut Context, params: P)
    where
        P: Into<DrawParams>,
    {
        self.current().draw(ctx, params)
    }

    fn current(&self) -> &Animation {
        match self.state {
            PlayerState::Idle => &self.idle,
            PlayerState::Running => &self.running,
        }
    }

    fn current_mut(&mut self) -> &mut Animation {
        match self.state {
            PlayerState::Idle => &mut self.idle,
            PlayerState::Running => &mut self.running,
        }
    }

    pub fn advance(&mut self, ctx: &Context) {
        self.current_mut().advance(ctx);
    }

    pub fn set_state(&mut self, state:PlayerState) {
        if self.state != state {
            debug!("restart: {:?}, {:?}", self.state, state);
            self.state = state;
            self.current_mut().restart();
        }
    }
}

pub struct Player {
    pub position: Vec2<f32>,
    pub animation: PlayerAnimation,
}

impl Player {
    pub fn new(ctx: &mut Context, position: Vec2<f32>) -> tetra::Result<Player> {
        Ok(Player {
            animation: PlayerAnimation::new(ctx)?,
            position
        })
    }
}
