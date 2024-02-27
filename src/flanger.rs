use crate::echo::Echo;
use crate::sine::SineWave;
// use crate::dummy::Dummy;
use crate::config::AppTrait;
// TODO: 
// * bug ?
// * use dummy as in_app or not
pub struct Flanger {
    sine_wave: SineWave, // TODO: WN etc.
    echo: Echo,
    nframes_delay: usize,
    lfo_index: f32,
    depth: f32,
}

impl Flanger {
    pub fn new(sample_rate: usize, sine_table_size:usize, freq: f32, gain:f32, lfo_index: f32, depth: f32, nframes_delay: usize, feedback: f32, port_pairs:usize, in_app: Box<dyn AppTrait>) -> Self {
        let sine_wave = SineWave::new(sample_rate, sine_table_size, None, freq, gain);
        // let echo = Echo::new(nframes_delay, feedback, port_pairs, Box::new(Dummy::new())); //in_app move to echo
        let echo = Echo::new(nframes_delay, feedback, port_pairs, in_app);
        Flanger {
            sine_wave,
            echo,
            nframes_delay,
            lfo_index,
            depth,
        }
    }
}

impl AppTrait for Flanger {
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32 {
        let lfo = self.sine_wave.tick(in_frame, port_index) * 0.5 + 0.5;
        self.echo.set_delay((self.nframes_delay as f32 * (1.0 - lfo * self.lfo_index)) as usize);
        self.echo.tick(in_frame, port_index) * self.depth
    }
    fn save_init(&mut self) {
        self.echo.save_init(); // echo.save -> in_app.save
        self.sine_wave.save_init();
    }
    fn load_init(&mut self) {
        self.echo.load_init();
        self.sine_wave.load_init();
    }
}
