#![no_std]
#![no_main]
use firefly_rust as ff;

#[no_mangle]
extern fn boot() {
    ff::AudioNode::ROOT.add_sine(ff::Freq::A4, 0.);
}
