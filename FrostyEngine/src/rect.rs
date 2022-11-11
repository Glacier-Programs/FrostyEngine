#[derive( Copy, Clone, Debug )]
pub struct Rect{
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Rect{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self{
        Self{
            x,
            y,
            width,
            height
        }
    }
}