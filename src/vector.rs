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