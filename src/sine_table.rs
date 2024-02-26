//! Calculate a sine table only once, look up sinusoid values later. 
//! ## TODO：
//! * Linear interpolation while keep using a small table ?
//! * TODO1 line 64.
use std::f32::consts::PI;
// use crate::get_type;

#[derive(Debug)]
pub struct SineTable {
    table: Vec<f32>,
}

impl SineTable {
    /// Returns a `SineTable` instance with the specified size.
    /// 
    /// # How to Calculate Your Table Size
    /// When the size is `1024`, the difference between two phases is approximately `0.006`. 
    /// For a `48kHz` sample rate, the precision is around `45Hz` `(0.006/[2*pi/48k])`.
    /// To achieve a precision of `10Hz` under a `48kHz` sample rate, 
    /// the phase difference needs to be less than `10*(2*pi/48k) = 0.0013`,
    /// which corresponds to a table size of `4096`.
    /// ## Quick Precision Notes for 48kHz and 44.1kHz
    /// * 1024 -> 45Hz \| 42Hz
    /// * 2048 -> 22Hz \| 21Hz
    /// * 4096 -> 11Hz \| 11Hz
    /// 
    /// # Examples:
    /// Create an instance:
    /// ```
    /// # use mydsp_jack::sine_table::SineTable;
    /// let size = 4096;
    /// let sinetable = SineTable::new(size);
    /// # let answer1 = sinetable.get_value(0.0);
    /// # let answer2 = sinetable.get_value(1.0 - 1.0 / size as f32);
    /// # debug_assert_eq!(0.0, answer1); // first value
    /// # debug_assert_eq!(-0.0015338097, answer2); // last value
    /// ```
    /// # Panics
    /// The function panics if `size` is not a positive unsigned integer.
    /// ```rust, should_panic
    /// # use mydsp_jack::sine_table::SineTable;
    /// let sinetable1 = SineTable::new(0);
    /// ```
    pub fn new(size: usize) -> SineTable {
        // if size <= 0{
        //     panic!("Panic: size of the sine table should be a positive unsigned integer (usize)");
        // } // do not need, because size: usize
        let mut table = Vec::with_capacity(size);
        for i in 0..size {
            let phase = 2.0 * PI * (i as f32) / (size as f32);
            // let x = phase.sin();
            // println!("{1}   {0}", x, get_type(&x)); // print whole sine table
            table.push(phase.sin());
        }
        SineTable { table }
    }
    /// Get the length of a SineTable instance.
    pub fn length(&self) -> usize {
        self.table.len()
    }
    /// Get the Value of a SineTable instance with normalized `phase` in range `[0,1)`.
    pub fn get_value(&self, phase: f32) -> f32 {
        let index = (self.length() as f32 * phase) as usize; 
        // TODO1: should we use this instead when dealing with low percision table? 
        // let index = if phase >= 1.0 { 0 } else { (self.length() as f32 * phase) as usize };
        self.table[index]
    }
}
