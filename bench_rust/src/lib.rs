use std::time::{Duration, Instant};
use std::env;

//#[cfg(feature="native")]
pub mod native_handle {
    use suite::*;
    use wasm_std::*;

    pub struct Sine {
        frames_per_call: u32,
    }
    
    impl Sine {
        pub fn new(sample_rate: f64, frames_per_call: u32) -> Self {
           // unsafe { intrinsics::alloc_and_set(frames_per_call); }
            sine_init(sample_rate);
            Self {
                frames_per_call,
            }
        }

        pub fn run(&self) {
            sine_compute(self.frames_per_call);
        }
    }

    pub struct Null {
        frames_per_call: u32,
    }
    
    impl Null {
        pub fn new(sample_rate: f64, frames_per_call: u32) -> Self {
            Self {
                frames_per_call,
            }
        }

        pub fn run(&self) {
            null(self.frames_per_call);
        }
    }
}

//#[cfg(feature="wasm")]
pub mod wasm_handle {
    use wasm_std::*;
    use wasmer_runtime::{imports,  Instance, compile, compile_with_config_with, func, Func, CompilerConfig };
    use wasmer_llvm_backend::LLVMCompiler;
    use wasmer_runtime_core::{compile_with, backend::MemoryBoundCheckMode};

    #[no_mangle]
    fn wrapper_write_f32(offset: i32, v: f32) {
        unsafe {
            intrinsics::write_f32(offset, v);
        }
    }

    pub struct Sine {
        frames_per_call: u32,
        #[allow(dead_code)]
        instance: Instance,
        compute: Func<'static, u32, ()>,
    }

    impl Sine {
        pub fn new(sample_rate: f64, frames_per_call: u32) -> Self {
            //unsafe { intrinsics::alloc_and_set(frames_per_call); }
            let mut compiler_config = CompilerConfig::default();
            compiler_config.memory_bound_check_mode = MemoryBoundCheckMode::Disable;

            let wasm_bytes = include_bytes!("../../suite/pkg/suite_bg.wasm");

            let import_object = imports! {
                "env" => {
                    "_write_f32" => func!(wrapper_write_f32),
                }
            };
            let instance = compile_with_config_with(wasm_bytes, compiler_config, &LLVMCompiler::new())
                .unwrap()
                .instantiate(&import_object)
                .unwrap();

            let init: Func<f64, ()> = instance.exports.get("sine_init").unwrap();
            init.call(sample_rate).unwrap();

            unsafe {
                let compute = instance.exports.get::<Func<'_, u32, ()>>("sine_compute").unwrap();
                let compute = std::mem::transmute::<Func<'_, u32, ()>, Func<'static, u32, ()>>(compute);
                Self {
                    frames_per_call,
                    instance,
                    compute,
                }
            }
        }

        pub fn run(&self) {
            self.compute.call(self.frames_per_call).unwrap();
        }
    }

    pub struct Null {
        frames_per_call: u32,
        #[allow(dead_code)]
        instance: Instance,
        compute: Func<'static, u32, ()>,
    }

    impl Null {
        pub fn new(sample_rate: f64, frames_per_call: u32) -> Self {
            //unsafe { intrinsics::alloc_and_set(frames_per_call); }

            let wasm_bytes = include_bytes!("../../suite/pkg/suite_bg.wasm");

            let import_object = imports! {
                "env" => {
                    "_write_f32" => func!(wrapper_write_f32),
                }
            };
            let instance = compile_with(wasm_bytes, &LLVMCompiler::new())
                .unwrap()
                .instantiate(&import_object)
                .unwrap();

            unsafe {
                let compute = instance.exports.get::<Func<'_, u32, ()>>("null").unwrap();
                let compute = std::mem::transmute::<Func<'_, u32, ()>, Func<'static, u32, ()>>(compute);
                Self {
                    frames_per_call,
                    instance,
                    compute,
                }
            }
        }

        pub fn run(&self) {
            self.compute.call(self.frames_per_call).unwrap();
        }
    }
}

//pub use wasm_handle::*;
//pub use native_handle::*;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         println!("{} buffer_size", args[0]);
//         return;
//     }

//     let s = args[1].clone();
//     let buffer_size = s.parse::<u32>().unwrap();
//     let iterations = (44_100.0 / buffer_size as f64) as usize;

//     //--------------------------------------
//     let h = Null::new(44_100., buffer_size);
//     println!("Running Null Benchmark");
//     let now = Instant::now();
//     h.run(iterations);
//     let elapsed = now.elapsed();
//     let m = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
//     if m == 0 {
//         let m = elapsed.subsec_nanos() as u64;
//         println!("Time taken for {} iterations: {} ns with a buffer size {}", iterations, m, buffer_size);
//     }
//     else {
//         println!("Time taken for {} iterations: {} ms with a buffer size {}", iterations, m, buffer_size);
//     }

//     //--------------------------------------
//     let h = Sine::new(44_100., buffer_size);
//     println!("Running Sine Benchmark");
//     let now = Instant::now();
//     h.run(iterations);
//     let elapsed = now.elapsed();

//     let m = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
//     if m == 0 {
//         let m = elapsed.subsec_nanos() as u64;
//         println!("Time taken for {} iterations: {} ns with a buffer size {}", iterations, m, buffer_size);
//     }
//     else {
//         println!("Time taken for {} iterations: {} ms with a buffer size {}", iterations, m, buffer_size);
//     }
// }
