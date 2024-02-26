//! Sinewave callback. Module `sine` of library crate `mydsp-jack`. 
//! 
//! # Examples:
//! Pass `output_ports` and `sample_rate` using jack::{AudioOut, Client}.
//! 
//! `phase`(initial phase) should be between [0.0, 1), and `freq`(frequency) and `gain`(volumn) should be positive.
//! 
//! Pass `None` to `phase/freq/gain` to indicate using default values `(phase=0.0, freq=440.0, gain=1.0)`
//! ```
//! let sine_wave = SineWave::new(output_ports, sample_rate, 4096, 0.99, 1000.0, 0.5);
//! // use `None` to indicate using default values
//! let sine_wave_default = SineWave::new(output_ports, sample_rate, 4096, None, None, None);
//! ```
//! # Notes:
//! ## Default values
//! To achieve using default parameters, I have thought about other methods like using Default trait, builder pattern or simply expose 
//! fields which contain default values like `pub phase,`, but `phase: impl Into<Option<f32>>` seems to be the most elegant.
//! 
//! ## Method new()
//! Rust do not have null type, so use `Option<f32>`, which contains `Some()`, `None`. 
//! Without `phase: impl Into<Option<f32>>` and `phase.into().unwrap_or(0.0)`, this usage will not be possible:
//! ```
//! let sine_wave = SineWave::new(output_ports, sample_rate, 4096, None, 1000.0, 0.5);
//! ```
//! However, values of `phase`, `freq`, `gain` should only be `float` or `None`. Integers are not supported. (Monomorphization)
//! 
//! ## TODO:
//! * add panics
//! * add auto-test and panics-test
//! * `phase`, `freq`, `gain` in new() take integer `(i32, u32)` as well. (use enum() instead?)
//! * impact of using Option<f32> (for default values) instead of f32 remains unclear.
// use crate::import::*;
use crate::sine_table::SineTable;
use crate::phasor::Phasor;
use crate::config::AppTrait;

#[derive(Debug)]
pub struct SineWave {
    sine_table: SineTable,
    // sine_table_size: usize,
    phasor: Phasor,
    phase: f32,
    // freq: f32, //frequency
    gain: f32, //volumn
    // sample_rate: f32,
}

impl SineWave {
    pub fn new(sample_rate: usize, sine_table_size: usize, phase_init: impl Into<Option<f32>>, freq: impl Into<Option<f32>>, gain: impl Into<Option<f32>>) -> SineWave {
        let phase_init = phase_init.into().unwrap_or(0.0);
        let freq = freq.into().unwrap_or(440.0);
        let gain = gain.into().unwrap_or(1.0);
        // let sample_rate = config.sample_rate;
        let phase_increment = freq / sample_rate as f32;
        SineWave {
            sine_table: SineTable::new(sine_table_size),
            // sine_table_size,
            phasor: Phasor::new(phase_init, phase_increment),
            phase: phase_init,
            // freq,
            gain,
            // sample_rate,
        }
    }
    // pub fn tick(&mut self) -> f32 {
    //     // let phase_increment = self.freq / sample_rate;
    //     self.phase = self.phasor.tick();
    //     self.sine_table.get_value(self.phase) * self.gain
    // }
}

// note that update phasor.phase NOT sine.phase
impl AppTrait for SineWave {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        self.phase = self.phasor.tick(in_frame, port_index);
        self.sine_table.get_value(self.phase) * self.gain
    }
    fn save_init(&mut self) { 
        self.phasor.save_init();
    }
    fn load_init(&mut self) {
        self.phasor.load_init();
    }
    // fn print_phase(&mut self) {
    //     println!("current_phase={}", self.phase);
    // }
}
