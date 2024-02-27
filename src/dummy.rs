use crate::config::AppTrait;

pub struct Dummy {}

impl Dummy {
    pub fn new() -> Self {
        Dummy {}
    }
}

impl AppTrait for Dummy {
    fn tick(&mut self, in_frame: f32, _port_index: usize) -> f32 {
        in_frame
    }
    fn save_init(&mut self) {
    }
    fn load_init(&mut self) {
    }
}
