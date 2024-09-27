use crate::vector::Vec2;
use crate::vector::Vec3;
use crate::vector::Position2d;
use crate::vector::Position3d;

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

    pub fn execute_2d_simulation(mass:f64, force:Vec2) -> Vec<Position2d> {
        let dt:f64 = 0.01;
        let mut particle = Particle2d::new(Vec2::new(0.0, 0.0));
        let mut position_vector: Vec<Position2d> = Vec::new();
    
        particle.apply_force(force, mass);
    
        for step in 0..1000{
            let time = step as f64 * dt;
    
            particle.update(dt);
    
            let position = Position2d::new(time, particle.position);
            position_vector.push(position);
        }
        
        return position_vector;
    }

    pub fn find_distance(&self, other_particle:&Particle2d) -> f64{
        let x_distance = (self.position.x - other_particle.position.x);
        let y_distance = (self.position.y - other_particle.position.y);
        
        (x_distance.powi(2) + y_distance.powi(2)).sqrt()
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

    pub fn execute_3d_simulation(mass:f64, force:Vec3) -> Vec<Position3d> {
        let dt:f64 = 0.01;
    
        let mut particle = Particle3d::new(Vec3::new(0.0, 0.0, 0.0));
        let mut position_vector: Vec<Position3d> = Vec::new();
    
        particle.apply_force(force, mass);
    
        for step in 0..1000{
            let time = step as f64 * dt;
    
            particle.update(dt);
            
            let position = Position3d::new(time, particle.position);
            position_vector.push(position);
        }
    
        return position_vector;
    }

    pub fn find_distance(&self, other_particle:&Particle3d) -> f64{
        let x_distance = self.position.x - other_particle.position.x;
        let y_distance = self.position.y - other_particle.position.y;
        let z_distance = self.position.z - other_particle.position.z;
        
        (x_distance.powi(2) + y_distance.powi(2) + z_distance.powi(2)).sqrt()
    }
}

