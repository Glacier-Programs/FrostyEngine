use instant;

pub struct TimeKeep{
    previous_frame_time: instant::Instant,
    frames: u64
}

impl TimeKeep{
    pub fn new() -> Self{
        Self{
            previous_frame_time: instant::Instant::now(),
            frames: 0u64
        }
    }

    pub fn tick(&mut self){
        self.frames += 1;
    }

    pub fn get_dt_as_secs(&mut self) -> f64{
        let now = instant::Instant::now();
        let dt: instant::Duration = now - self.previous_frame_time;
        self.previous_frame_time = now;
        dt.as_secs_f64()
    }
}