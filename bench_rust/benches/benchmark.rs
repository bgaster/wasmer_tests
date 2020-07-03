use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use bench_rust::*;

fn criterion_benchmark(c: &mut Criterion) {
    let buffer_size = 64;
    let iterations = (44_100.0 / buffer_size as f64) as usize;
    
    for buffer_size in &[1 as u32, 16, 32, 64, 128] {

        let buffer_size_str = (*buffer_size).to_string();

        // native tests - buffer size
        let native_null = native_handle::Null::new(44_100., *buffer_size);
        c.bench_function(&format!("native null {}", buffer_size_str)[..] , |b| b.iter(|| native_null.run()));
        let native_sine = native_handle::Sine::new(44_100., *buffer_size);
        c.bench_function(&format!("native sine {}", buffer_size_str)[..], |b| b.iter(|| native_sine.run()));

        // wasm tests

        let wasm_null = wasm_handle::Null::new(44_100., *buffer_size);
        c.bench_function(&format!("wasm null {}", buffer_size_str)[..], |b| b.iter(|| wasm_null.run()));

        let wasm_sine = wasm_handle::Sine::new(44_100., *buffer_size);
        c.bench_function(&format!("wasm sine {}", buffer_size_str)[..], |b| b.iter(|| wasm_sine.run()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);