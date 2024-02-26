use crate::config::AppTrait;

pub struct Smooth {
    previous_output: f32,
    previous_output_save: f32,
    smoothing_factor: f32,
    in_app: Box<dyn AppTrait>,
}

impl Smooth {
    pub fn new(smoothing_factor: f32, in_app: Box<dyn AppTrait>) -> Self {
        Smooth {
            previous_output: 0.0,
            previous_output_save: 0.0,
            smoothing_factor,
            in_app,
        }
    }
}

impl AppTrait for Smooth {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let in_frame = in_frame + self.in_app.tick(in_frame, port_index);
        let output = (1.0 - self.smoothing_factor) * in_frame + self.smoothing_factor * self.previous_output;
        // println!("{}, {}", port_index, output);
        self.previous_output = output;
        output
    }
    fn save_init(&mut self) {
        // println!("smooth_save:     pre{}, save{}", self.previous_output, self.previous_output_save);
        self.previous_output_save = self.previous_output;
        self.in_app.save_init();
    }
    fn load_init(&mut self) {
        // println!("smooth_load：    pre{}, save{}", self.previous_output, self.previous_output_save);
        self.previous_output = self.previous_output_save;
        self.in_app.load_init();
    }
}