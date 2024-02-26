/*! lib like mydsp in C++ 
 * 
 * # Notes
 * ## #[derive(Debug)]
 * All structs implant `#[derive(Debug)]`, to use `std::fmt formatting` traits, derive from Debug trait.
 * 
 * Usage: `println?("{:?}", all_type)` or `println?("{:#?}", all_type)` to display any type without impl `fmt::Display` manually.
 * 
 * 
*/

/// In each module, just: `use crate::import::*;`
pub mod import{
	pub(crate) use
	// {
        jack::{Port, AudioIn, AudioOut, Client, Control, ProcessHandler, ProcessScope};
        // std     :: error::Error,
	// };
}

pub mod config;
#[doc(alias = "sinetable")]
#[doc(alias = "table")]
pub mod sine_table;
#[doc(alias = "phase")]
pub mod phasor;
#[doc(alias = "sinewave")]
pub mod sine;

pub mod echo;
#[doc(alias = "PWM")]
pub mod pwm;

pub mod noise;

pub mod smooth;
pub mod one_zero;
pub mod distortion;

pub fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}
