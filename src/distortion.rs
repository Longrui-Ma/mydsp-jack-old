use crate::config::AppTrait;

pub struct Distortion {
    drive: f32,
    offset: f32,
    gain: f32,
    in_app: Box<dyn AppTrait>,
}

impl Distortion {
    pub fn new(drive: f32, offset: f32, gain: f32, in_app: Box<dyn AppTrait>,) -> Self {
        Distortion {
            drive,
            offset,
            gain,
            in_app,
        }
    }
    fn cubic(&self, x: f32) -> f32 {
        x - x.powi(3) / 3.0
    }
}

impl AppTrait for Distortion {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let in_frame = in_frame + self.in_app.tick(in_frame, port_index);
        let mut output = in_frame * 10.0_f32.powf(2.0 * self.drive) + self.offset;
        output = output.clamp(-1.0, 1.0); //limit output in [-1.0, 1.0]
        output = self.cubic(output);
        output * self.gain
    }
    fn save_init(&mut self) {
        self.in_app.save_init();
    }
    fn load_init(&mut self) {
        self.in_app.load_init();
    }
}
