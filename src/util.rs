use sdl3::libc::{RAND_MAX, rand, srand};
use std::ffi::c_uint;

pub fn seed_random(seed: u32) {
    unsafe {
        srand(seed as c_uint);
    }
}

pub fn random(min: f32, max: f32) -> f32 {
    unsafe { rand() as f32 / RAND_MAX as f32 * (max - min) + min }
}
