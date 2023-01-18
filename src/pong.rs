use nannou::prelude::*;

pub const PLAYER_LENGTH: f32 = 100.;
pub const PLAYER_THICKNESS: f32 = 10.;
pub const BALL_RADIUS: f32 = 10.0;

pub const GAME_WIDTH: f32 = 600.0;
pub const GAME_HEIGHT: f32 = 400.0;

pub const BALL_SPEED: f32 = 100.0;

pub struct Pong {
    pub dim: Vec2,
    pub ball: Ball,
    pub players: [Player; 2]
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            dim: vec2(GAME_WIDTH as f32, GAME_HEIGHT as f32),
            ball: Ball::new(vec2(0.0, 0.0), BALL_RADIUS),
            players: [Player::new(0), Player::new(1)]
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.ball.pos.y.abs() + self.ball.rad > self.dim.y / 2.0 {
            self.ball.vel.y *= -1.0;
        }

        for player in &self.players {
            if self.ball.pos.x.abs() + self.ball.rad > player.pos.x.abs() - PLAYER_THICKNESS / 2.0 {
                if self.ball.pos.y < player.pos.y + PLAYER_LENGTH / 2.0 && self.ball.pos.y > player.pos.y - PLAYER_LENGTH / 2.0 {
                    self.ball.vel.x *= -1.0;
                    self.ball.vel.y += player.vel.y;
                    break
                }
            }
        }

        self.ball.update(dt);
    }
}

pub struct Ball {
    pub rad: f32,
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2, radius: f32) -> Ball {
        Ball {
            rad: radius,
            pos,
            vel: vec2(BALL_SPEED * 0.8, BALL_SPEED * 0.6),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }
}

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2
}

impl Player {
    pub fn new(num: usize) -> Player {
        Player {
            pos: vec2((num as f32 * 2.0 - 1.0) * GAME_WIDTH / 2.0 * 0.9, 0.0),
            vel: vec2(0.0, 0.0)
        }
    }
}