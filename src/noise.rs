//! Generate noise.
//!
//! Uniform::new and Uniform::new_inclusive construct a uniform distribution sampling from the given range; 
//!     these functions may do extra work up front to make sampling of multiple values faster.
//! https://docs.rs/rand/0.6.5/rand/distributions/uniform/struct.Uniform.html
use crate::config::AppTrait;
// use rand::Rng;
use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
pub struct WhiteNoise {
    gain: f32,
    between: Uniform<f32>,
}

impl WhiteNoise {
    pub fn new(gain: f32) -> WhiteNoise {
        WhiteNoise {
            gain,
            between: Uniform::from(-1.0..1.0),
        }
    }
}

impl AppTrait for WhiteNoise {
    fn tick(&mut self, _in_frame: f32, _port_index: usize) -> f32 {
        // let mut rng = rand::thread_rng(); 
        // self.gain * rng.gen::<f32>() * 2.0 - 1.0 // [-1.0, 1.0)
        let mut rng = rand::thread_rng();
        self.gain * self.between.sample(&mut rng) // [-1.0, 1.0)
    }
    fn save_init(&mut self) {
    }
    fn load_init(&mut self) {
    }
}