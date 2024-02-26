//! Configuration for input, output ports and sample rate.
// TODO: verify in_buffer size = out_buffer, and same numbers of input_ports and output_ports 
use crate::import::*;

// #[derive(Debug)]
pub struct Config{
    pub input_ports: Vec<Port<AudioIn>>,
    pub output_ports: Vec<Port<AudioOut>>,
    // pub sample_rate: f32,
    pub app: Box<dyn AppTrait>, //trait object
}

impl Config{
    // pub fn new(client: &Client, input_ports: Vec<Port<AudioIn>>, output_ports: Vec<Port<AudioOut>>, app: Box<dyn AppTrait>) -> Config {
    pub fn new(input_ports: Vec<Port<AudioIn>>, output_ports: Vec<Port<AudioOut>>, app: Box<dyn AppTrait>) -> Config {
        // let sample_rate = client.sample_rate() as f32;
        // let sample_rate = Config::get_sample_rate(client);
        Config {
            input_ports,
            output_ports,
            // sample_rate,
            app,
        }
    }
    // pub fn get_sample_rate(client: &jack::Client) -> f32{
    //     client.sample_rate() as f32
    // }
}

pub trait AppTrait: Send + Sync{
    fn tick(&mut self, in_frame: f32, port_index: usize) -> f32;
    fn save_init(&mut self);
    fn load_init(&mut self);
    // fn print_phase(&mut self);
}

impl ProcessHandler for Config {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        self.app.save_init();
        // println!("----------------{}", ps.n_frames());
        // for port in &mut self.output_ports {
        //     self.app.load_init();
        //     let buffer = port.as_mut_slice(ps);
        //     for frame in buffer.iter_mut() {
        //         *frame = self.app.tick();
        //     }
        for (port_index, (in_port, out_port)) in self.input_ports.iter().zip(&mut self.output_ports).enumerate() {
            self.app.load_init();
            let in_buffer = in_port.as_slice(ps);
            let out_buffer = out_port.as_mut_slice(ps);
            for (in_frame, out_frame) in in_buffer.iter().zip(out_buffer.iter_mut()) { 
                *out_frame = self.app.tick(*in_frame, port_index);
            }
            // println!("---{} port end", port_index);
        }
            // self.app.print_phase();
        // }
        // self.app.print_phase();
        Control::Continue
    }
}

// pub trait Init {
//     fn get_sample_rate(&self) -> f32;
// }

// impl Init for Config {
//     fn get_sample_rate(&self) -> f32 {
//         self.get_sample_rate()
//     }
// }
