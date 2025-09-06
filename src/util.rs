use sdl3::libc::{RAND_MAX, rand, srand};

pub fn seed_random(seed: u32) {
    unsafe {
        srand(seed);
    }
}

pub fn random(min: f32, max: f32) -> f32 {
    unsafe { rand() as f32 / RAND_MAX as f32 * (max - min) + min }
}
