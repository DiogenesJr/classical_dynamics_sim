mod vector;
mod particle;

use vector::Vec2;
use vector::Vec3;

use particle::Particle2d;
use particle::Position2d;
use particle::Particle3d;
use particle::Position3d;

use core::error;
use std::vec;
use std::vec::Vec;
use std::error::Error;
use std::fs::File;
use csv::Writer;


fn main() {
    println!("Executing 2d simulation:");
    match execute_2d_simulation() {
        Ok(position_vector_2d) => {
            output_2d_data_to_csv("position_2d.csv", position_vector_2d).unwrap();
        },
        Err(e) => {
            println!("Error: {e}");
        }
    }

    println!("");
    
    println!("Executing 3d simulation:");
    match execute_3d_simulation(){
        Ok(position_vector_3d) => {
            output_3d_data_to_csv("position_3d.csv", position_vector_3d).unwrap();
        },
        Err(e) => {
            println!("Error: {e}");
        }
    }
}

fn execute_3d_simulation() -> Result<Vec<Position3d>, Box<dyn Error>> {
    let dt:f64 = 0.01;

    let mut particle = Particle3d::new(Vec3::new(0.0, 0.0, 0.0));
    let mut position_vector: Vec<Position3d> = Vec::new();

    let force = Vec3::new(7.54,2.2,1.54);
    let mass = 1.0;

    particle.apply_force(force, mass);

    for step in 0..1000{
        let time = step as f64 * dt;

        particle.update(dt);
        
        let position = Position3d::new(time, particle.position);
        position_vector.push(position);
    }

    Ok(position_vector)
}

fn execute_2d_simulation() -> Result<Vec<Position2d>, Box<dyn Error>> {
    let dt:f64 = 0.01;
    let mut particle = Particle2d::new(Vec2::new(0.0, 0.0));
    let mut position_vector: Vec<Position2d> = Vec::new();

    let force = Vec2::new(7.54,2.2);
    let mass = 1.0;

    particle.apply_force(force, mass);

    for step in 0..1000{
        let time = step as f64 * dt;

        particle.update(dt);

        let position = Position2d::new(time, particle.position);
        position_vector.push(position);
    }
    
    Ok(position_vector)
}

fn output_2d_data_to_csv(file_name:&str, position_vector:Vec<Position2d>) -> Result<(), Box<dyn Error>>{
    let file =  File::create(file_name)?;

    let mut writer = Writer::from_writer(file);
    writer.write_record(&["time","x","y"])?;

    for position in position_vector{
        writer.write_record(&[
            position.time.to_string(),
            position.position.x.to_string(),
            position.position.y.to_string()
        ])?;
    }
    writer.flush()?;

    Ok(())
}

fn output_3d_data_to_csv(file_name:&str, position_vector:Vec<Position3d>) -> Result<(), Box<dyn Error>>{
    let file =  File::create(file_name)?;

    let mut writer = Writer::from_writer(file);
    writer.write_record(&["time","x","y","z"])?;

    for position in position_vector{
        writer.write_record(&[
            position.time.to_string(),
            position.position.x.to_string(),
            position.position.y.to_string(),
            position.position.z.to_string(),
        ])?;
    }
    writer.flush()?;

    Ok(())
}