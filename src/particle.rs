use crate::vector::Vec2;
use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Particle2d{
    pub position:Vec2,
    pub previous_position:Vec2,
    pub acceleration:Vec2
}

impl Particle2d {
    pub fn new(position:Vec2) -> Particle2d {
        Particle2d {
            position,
            previous_position: position,
            acceleration: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt:f64){
        let temp = self.position;

        self.position = self.position
            .add(self.position.add(self.previous_position.scale(-1.0)))
            .add(self.acceleration.scale(dt*dt));

        self.previous_position = temp;
    }

    pub fn apply_force(&mut self, force:Vec2, mass:f64){
        self.acceleration = force.scale(1.0/mass);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position2d{
    pub time:f64,
    pub position:Vec2,
}

impl Position2d{
    pub fn new(time:f64, position:Vec2)->Position2d{
        Position2d{
            time,
            position
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Particle3d{
    pub position:Vec3,
    pub previous_position:Vec3,
    pub acceleration:Vec3
}

impl Particle3d{
    pub fn new(position:Vec3) -> Particle3d {
        Particle3d{
            position,
            previous_position: position,
            acceleration: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt:f64){
        let temp = self.position;

        self.position = self.position
            .add(self.position.add(self.previous_position.scale(-1.0)))
            .add(self.acceleration.scale(dt*dt));

        self.previous_position = temp;
    }

    pub fn apply_force(&mut self, force:Vec3, mass:f64){
        self.acceleration = force.scale(1.0/mass);
    }
}

pub struct Position3d{
    pub time:f64,
    pub position:Vec3,
}

impl Position3d{
    pub fn new(time:f64, position:Vec3) -> Position3d{
        Position3d { time: time, position: position }
    }
}