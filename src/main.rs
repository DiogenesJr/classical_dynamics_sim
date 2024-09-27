mod vector;
mod particle;

use vector::Vec2;
use vector::Vec3;

use particle::Particle2d;
use particle::Particle3d;

use crate::vector::Position2d;
use crate::vector::Position3d;

fn main() {
    println!("Executing 2d simulation:");
    
    let force2d = Vec2::new(1.34,2.45);
    let mass = 1.65;
    let position_vector_2d:Vec<Position2d> = Particle2d::execute_2d_simulation(mass, force2d);
    Position2d::output_2d_to_console(&position_vector_2d);

    println!("");

    println!("Executing 3d simulation:");
    let force3d = Vec3::new(1.34, 2.45, 1.74);
    let position_vector_3d:Vec<Position3d> = Particle3d::execute_3d_simulation(mass,force3d);
    Position3d::output_3d_to_console(&position_vector_3d);
}



