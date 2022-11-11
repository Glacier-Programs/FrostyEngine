use wgpu;

pub fn rgb_to_wgpu_col(r: i32, g: i32, b: i32, a: i32) -> wgpu::Color{
    let new_r = (r/255) as f64;
    let new_g = (g/255) as f64;
    let new_b = (b/255) as f64;
    let new_a = (a/255) as f64;
    wgpu::Color{r: new_r, g: new_g, b: new_b, a: new_a}
}