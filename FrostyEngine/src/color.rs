pub struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color{
    pub fn new<S>(r: S, g: S, b: S, a: S) -> Self
    where S: Into<u8> {
        Self{
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: a.into()
        }
    }

    pub fn as_f64(&self) -> [f64;4]{
        // turns from 0-255 to 0-1
        [ self.r as f64/255.0, self.g as f64/255.0, self.b as f64/255.0, self.a as f64/255.0 ]
    }

    pub fn as_f32(&self) -> [f32;4]{
        // turns from 0-255 to 0-1
        [ self.r as f32/255.0, self.g as f32/255.0, self.b as f32/255.0, self.a as f32/255.0 ]
    }

    pub fn to_wgpu_color(&self) -> wgpu::Color{
        let cols = self.as_f64();
        wgpu::Color{
            r: cols[0],
            g: cols[1],
            b: cols[2],
            a: cols[3]
        }
    }
}

pub mod colors{
    use crate::color::Color;
    pub const RED: Color = Color{ r: 255, g: 0, b: 0, a: 255};
    pub const GREEN: Color = Color{ r: 0, g: 255, b: 0, a: 255};
    pub const BLUE: Color = Color{ r: 0, g: 0, b: 255, a: 255};
    pub const WHITE: Color = Color{ r: 255, g: 255, b: 255, a: 255};
    pub const BLACK: Color = Color{ r: 0, g: 0, b: 0, a: 255};
}