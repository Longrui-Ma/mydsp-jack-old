// Karplus Strong
use crate::config::AppTrait;
use crate::one_zero::OneZero;
// use crate::dummy::Dummy;
// TODO: 
// * bug ?
// * use dummy or not
pub struct KS {
    delay_buffers: Vec<Vec<f32>>, 
    read_indices: Vec<usize>, 
    write_indices: Vec<usize>, 
    feedback: f32,
    delay_length: usize,
    is_triggered: bool,
    one_zero: OneZero, 
}

impl KS {
    pub fn new(sample_rate: usize, feedback: f32, freq: f32, b1: f32, port_pairs: usize, in_app: Box<dyn AppTrait>) -> Self {
        let delay_length = (sample_rate as f32 / freq).round() as usize;
        KS {
            delay_buffers: vec![vec![0.0; delay_length]; port_pairs],
            read_indices: vec![delay_length - 1; port_pairs],
            write_indices: vec![0; port_pairs],
            feedback,
            delay_length,
            is_triggered: false,
            // one_zero: OneZero::new(b1, Box::new(Dummy::new())),
            one_zero: OneZero::new(b1, in_app),
        }
    }
    pub fn trigger(&mut self) {
        self.is_triggered = true;
    }
}

impl AppTrait for KS {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let excitation = if self.is_triggered { self.is_triggered = false; 1.0 } else { 0.0 };
        let input = in_frame + excitation + self.delay_buffers[port_index][self.read_indices[port_index]];
        let filtered_input = self.one_zero.tick(input, port_index);
        let output = filtered_input + self.feedback * self.delay_buffers[port_index][self.read_indices[port_index]]; 
        self.delay_buffers[port_index][self.write_indices[port_index]] = output;
        self.read_indices[port_index] = (self.read_indices[port_index] + 1) % self.delay_length;
        self.write_indices[port_index] = (self.write_indices[port_index] + 1) % self.delay_length;
        output
    }
    fn save_init(&mut self) {
        self.one_zero.save_init(); 
    }
    fn load_init(&mut self) {
        self.one_zero.load_init();
    }
}
