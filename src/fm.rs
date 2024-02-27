use crate::config::AppTrait;
use crate::phasor::Phasor;
use crate::sine_table::SineTable;

#[derive(Debug)]
pub struct Fm {
    c_phasor: Phasor, // carrier
    m_phasor: Phasor, // modulator
    fc: f32, // carrier_frequency
    mod_index: f32, // modulation index
    gain: f32,
    sine_table: SineTable,
}

impl Fm {
    pub fn new(sample_rate: usize, sine_table_size: usize, fc: f32, fm: f32, mod_index: f32, gain: f32) -> Self {
        Fm {
            c_phasor: Phasor::new(0.0, fc / sample_rate as f32),
            m_phasor: Phasor::new(0.0, fm / sample_rate as f32),
            fc,
            mod_index,
            gain,
            sine_table: SineTable::new(sine_table_size),
        }
    }
    // pub fn set_fc(&mut self, fc: f32) {
    //     self.fc = fc;
    //     self.c_phasor.set_phase_increment(fc / self.sine_table.length() as f32);
    // }
    // pub fn set_fm(&mut self, fm: f32) {
    //     self.m_phasor.set_phase_increment(fm / self.sine_table.length() as f32);
    // }
}

impl AppTrait for Fm {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let m_phase = self.m_phasor.tick(in_frame, port_index);
        let modulator: f32 = self.sine_table.get_value(m_phase);
        // let original_increment = self.fc / self.sine_table.length() as f32;
        self.c_phasor.set_phase_increment((self.fc + modulator * self.mod_index) / self.sine_table.length() as f32);
        let c_phase = self.c_phasor.tick(in_frame, port_index);
        // self.c_phasor.set_phase_increment(original_increment);
        self.sine_table.get_value(c_phase) * self.gain
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
