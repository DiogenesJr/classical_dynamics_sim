use std::vec::Vec;
use std::error::Error;
use std::fs::File;
use csv::Writer;


#[derive(Debug, Copy, Clone)]
pub struct Vec2{
    pub x:f64,
    pub y:f64
}

impl Vec2{
    pub fn new(x:f64, y:f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn add(&self, other:Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, scalar:f64)-> Vec2{
        Vec2 { 
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3{
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vec3{
    pub fn new(x:f64, y:f64, z:f64) -> Vec3{
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }
    
    pub fn add(&self, other: Vec3) -> Vec3{
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z,
         }
    }

    pub fn scale(&self, scalar:f64) -> Vec3{
        Vec3 { 
            x: self.x * scalar, 
            y: self.y * scalar, 
            z: self.z * scalar,
        }

    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position3d{
    pub time:f64,
    pub position:Vec3,
}

impl Position3d{
    pub fn new(time:f64, position:Vec3) -> Position3d{
        Position3d { time: time, position: position }
    }

    pub fn output_3d_to_console(position_vector:&Vec<Position3d>){
        for position in position_vector{
            println!("Time: {:?} Position: {:?}", position.time, position.position)
        }
    }
    
    pub fn output_3d_data_to_csv(file_name:&str, position_vector:&Vec<Position3d>) -> Result<(), Box<dyn Error>>{
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

    pub fn output_2d_data_to_csv(file_name:&str, position_vector:&Vec<Position2d>) -> Result<(), Box<dyn Error>>{
        let file =  File::create(file_name)?;
    
        let mut writer = Writer::from_writer(file);
        writer.write_record(&["time","x","y","z"])?;
    
        for position in position_vector{
            writer.write_record(&[
                position.time.to_string(),
                position.position.x.to_string(),
                position.position.y.to_string(),
            ])?;
        }
        writer.flush()?;
    
        Ok(())
    }
    
    pub fn output_2d_to_console(position_vector:&Vec<Position2d>){
        for position in position_vector{
            println!("Time: {:?} Position: {:?}",position.time, position.position)
        }
    }
}