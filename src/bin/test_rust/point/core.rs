// Struct and impl are basically Rust's version of classes
pub struct Point {
    x: f32,
    y: f32,
}
// implementation
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y } 
    }

    pub fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}