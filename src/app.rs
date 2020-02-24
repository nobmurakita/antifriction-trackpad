use crate::mouse_cursor;
use std::time::Instant;

#[derive(Debug)]
pub struct App {
    pos1: Option<Position>,
    pos2: Option<Position>,
    vel: Option<Velocity>,
    last_tick: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            pos1: None,
            pos2: None,
            vel: None,
            last_tick: Instant::now()    
        }
    }
}

impl App {
    pub fn touch(&mut self, t: f64) {
        let (x, y) = mouse_cursor::get_position();
        if let Some(pos2) = self.pos2 {
            if pos2.x != x || pos2.y != y {
                self.pos1 = self.pos2;
            }
        }
        self.pos2 = Some(Position { x, y, t });
        self.vel = None;
    }

    pub fn release(&mut self) {
        if let (Some(pos1), Some(pos2)) = (self.pos1, self.pos2) {
            let dx = pos2.x - pos1.x;
            let dy = pos2.y - pos1.y;
            let dt = pos2.t - pos1.t;
            self.vel = Some(Velocity { x: dx / dt, y: dy / dt });
            self.pos1 = None;
            self.pos2 = None;
        }
    }

    pub fn tick(&mut self) {
        let prev_tick = self.last_tick;
        self.last_tick = Instant::now();

        if let Some(mut vel) = self.vel {
            // カーソル移動
            let (x, y) = mouse_cursor::get_position();
            let dt = (self.last_tick - prev_tick).as_secs_f64();
            mouse_cursor::set_position(x + vel.x * dt, y + vel.y * dt);

            // 減速
            vel.multiply(1.0 - 10.0 * dt);
            if vel.length() < 10.0 {
                self.vel = None;
            } else {
                self.vel = Some(vel);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
    t: f64,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: f64,
    y: f64,
}

impl Velocity {
    fn multiply(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
    }

    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
