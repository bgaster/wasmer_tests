use wasm_bindgen::prelude::*;
use std::f64::consts::PI;

use wasm_std::*;

// const MAX_MEMORY_SIZE: usize = 1024 * 2;
// static mut MEMORY: [f32;MAX_MEMORY_SIZE] = [0.;MAX_MEMORY_SIZE];

//-----------------------------------------------------------------------------
// SINE BENCHMARK
//-----------------------------------------------------------------------------

const TABLE_SIZE: usize = 200;

static mut SINE: [f32;TABLE_SIZE] = [0.0; TABLE_SIZE];
static mut LEFT_PHASE: usize = 0;
static mut RIGHT_PHASE: usize = 0;
static mut SAMPLE_RATE: f64 = 0.0;

#[wasm_bindgen]
pub fn sine_init(sample_rate: f64) {
    unsafe {
        SAMPLE_RATE = sample_rate;
    }
    // craete sine table
    for i in 0..TABLE_SIZE {
        unsafe {
            SINE[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
        }
    }
}

#[wasm_bindgen]
pub fn sine_compute(frames: u32) {
    let mut idx = 0;
    for _ in 0..frames {
        unsafe {
            intrinsics::write_f32(idx, SINE[LEFT_PHASE]);
            intrinsics::write_f32(idx + 1, SINE[RIGHT_PHASE]);
            // MEMORY[idx] = SINE[LEFT_PHASE];
            // MEMORY[idx+1] = SINE[LEFT_PHASE];

            LEFT_PHASE += 1;
            if LEFT_PHASE >= TABLE_SIZE {
                LEFT_PHASE -= TABLE_SIZE;
            }
            RIGHT_PHASE += 3;
            if RIGHT_PHASE >= TABLE_SIZE {
                RIGHT_PHASE -= TABLE_SIZE;
            }
            idx += 2;
        }
    }
}

#[wasm_bindgen]
pub fn null(_frames: u32) {
}