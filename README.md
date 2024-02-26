# Rust Audio Processing @ INSA Lyon -- mydsp Repository
This library is developed with the aim of setting up a [5th year course option](https://embaudio.grame.fr/) around audio in Rust (@INSA Lyon - Département Télécommunications, Services et Usages).

The crate `mydsp-jack` written in Rust reproduces the functionalities found in [the original C++ `mydsp` library](https://github.com/grame-cncm/embaudio/tree/master/examples/teensy/libraries/mydsp), utilizing a [jack bindings for rust](https://github.com/RustAudio/rust-jack). 

Given the ease with which connections and modifications between different module inputs and outputs can be managed within GUI interfaces like qjackctl or Catia, akin to assembling LEGO, features enabling the simultaneous generation of multiple inputs and outputs for multichannel support have been integrated. The detailed design behind the `mydsp-jack` library is available in another [repository](https://github.com/Longrui-Ma/rust-audio-example).

## Dependencies
* [rust-jack](https://github.com/RustAudio/rust-jack)

## TODOs
* complet am.rs fm.rs flanger.rs ks.rs.
* In readme explain lib architecture.
* add examples in https://github.com/Longrui-Ma/rust-audio-example.
* doc test in lib modules
* to crate.io ?