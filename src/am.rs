use crate::config::AppTrait;
use crate::phasor::Phasor;
use crate::sine_table::SineTable;

#[derive(Debug)]
pub struct Am {
    c_phasor: Phasor, // carrier
    m_phasor: Phasor, // modulator
    mod_index: f32,   // modulation index
    gain: f32,
    sine_table: SineTable,
}

impl Am {
    pub fn new(sample_rate: usize, sine_table_size: usize, fc: f32, fm: f32, mod_index: f32, gain: f32) -> Self {
        Am {
            c_phasor: Phasor::new(0.0, fc / sample_rate as f32),
            m_phasor: Phasor::new(0.0, fm / sample_rate as f32),
            mod_index,
            gain,
            sine_table: SineTable::new(sine_table_size),
        }
    }
}

impl AppTrait for Am {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let c_phase = self.c_phasor.tick(in_frame, port_index);
        let m_phase = self.m_phasor.tick(in_frame, port_index);
        let pos_mod = self.sine_table.get_value(m_phase) * 0.5 + 0.5;
        self.sine_table.get_value(c_phase) * (1.0 - pos_mod * self.mod_index) * self.gain
    }
    fn save_init(&mut self) {
        self.c_phasor.save_init();
        self.m_phasor.save_init();
    }
    fn load_init(&mut self) {
        self.c_phasor.load_init();
        self.m_phasor.load_init();
    }
}
