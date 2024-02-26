use crate::config::AppTrait;

pub struct OneZero {
    previous_input: f32,
    previous_input_save: f32,
    b1: f32,             // filter coef
    in_app: Box<dyn AppTrait>,
}

impl OneZero {
    pub fn new(b1: f32, in_app: Box<dyn AppTrait>) -> Self {
        OneZero {
            previous_input: 0.0,
            previous_input_save: 0.0,
            b1,
            in_app,
        }
    }
}

impl AppTrait for OneZero {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let in_frame = in_frame + self.in_app.tick(in_frame, port_index);
        let output = in_frame + self.previous_input * self.b1;
        self.previous_input = in_frame;
        output * 0.5
    }
    fn save_init(&mut self) {
        self.previous_input_save = self.previous_input;
        self.in_app.save_init();
    }
    fn load_init(&mut self) {
        self.previous_input = self.previous_input_save;
        self.in_app.load_init();
    }
}
