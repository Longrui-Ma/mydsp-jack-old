# Rust Audio Processing @ INSA Lyon -- mydsp Repository
This library is developed with the aim of setting up a [5th year course option](https://embaudio.grame.fr/) around audio in Rust (@INSA Lyon - Département Télécommunications, Services et Usages).

The crate `mydsp-jack` written in Rust reproduces the functionalities found in [the original C++ `mydsp` library](https://github.com/grame-cncm/embaudio/tree/master/examples/teensy/libraries/mydsp), utilizing a [jack bindings for rust](https://github.com/RustAudio/rust-jack). 

Given the ease with which connections and modifications between different module inputs and outputs can be managed within GUI interfaces like qjackctl or Catia, akin to assembling LEGO, features enabling the simultaneous generation of multiple inputs and outputs for multichannel support have been integrated. The detailed design behind the `mydsp-jack` library is available in a separate [repository](https://github.com/Longrui-Ma/rust-audio-example).

## Dependencies
* [rust-jack](https://github.com/RustAudio/rust-jack)
## A quick demo
1. Install **Jack** on OS.
2. Download [***demo.rs***](https://github.com/Longrui-Ma/rust-audio-example/blob/main/rust-jack/mydsp-jack-conception/demo.rs) and follow instructions in ***demo.rs***.
## Architecture Overview
### .rs files relations
1. ***lib.rs***  consolidates all modules.
2. Then all modules are then processed through ***config.rs***.
3. The dependency relations among the rest of the modules are as follows (`<-` means dependecy):
- `am.rs`/`fm.rs`/`sine.rs` <- `phasor.rs` <- `sine_table.rs`
- `flanger.rs` <- `echo.rs`/`sine.rs` 
- `ks.rs` <- `one_zero.rs`
- Other modules operate independently.
## General Concept of `mydsp-jack` Architecture
### `config.rs`
***config.rs*** serves as an abstraction layer for all processes in audio processing, utilizing traits. It acts as an intermediary layer to fulfill the `ProcessHandler` trait required by the `rust-jack` crate, allowing instances of the config struct to be integrated into Jack's real-time processing framework.

To manage multichannel support with multiple inputs and outputs, a `for` loop iterates over each pair of ports (with one input and one output port forming a pair). It ensures that initial values of shared parameters, like `phase`, are uniform across pairs. The values of these shared parameters are preserved upon exit. Consequently, a trait `AppTrait` is implemented in ***config.rs*** and subsequently in all modules.

The common process for each pair involves reading from the input frame, then writing to the output frame. The audio processing that occurs between reading and writing is encapsulated by `app.tick()`, where app could be a struct in the modules or a struct that wraps other structs to create an "app chain". The `tick()` method is implemented across all structs to manage each frame. (The ***sine_table.rs*** does not implement `tick()` as table lookup is considered independent of audio processing and can operate at a higher frequency.)

In each module, `tick()` forwards parameters to the next `tick()` in the sequence, and `save_init()`/`load_init()` methods pass **save/load** commands to the lower layer.

Detailed techniques and examples will be provided in a separate [repository](https://github.com/Longrui-Ma/rust-audio-example)(documentation coming soon).
## TODOs
* problem with flanger.rs and ks.rs (bug + need dummy?)
* default values
* manuel #[derive(Debug)] for middle layers
* doc test in lib modules
* to crate.io ?