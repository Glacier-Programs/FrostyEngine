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

    pub fn as_decimal(&self) -> [f32;4]{
        // turns from 0-255 to 0-1
        [ self.r as f64/255f64, self.g as f64/255f64, self.b as f64/255f64, self.a as f64/255f64 ]
    }

    pub fn to_wgpu_color(&self) -> wgpu::Color{
        let cols = self.as_decimal();
        wgpu::Color{
            r: cols[0],
            g: cols[1],
            b: cols[2],
            a: cols[3]
        }
    }
}