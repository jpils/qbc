use std::path::Path;

use tch::{CModule, Tensor, Kind, Device};
use anyhow::{Context, Result};
use crate::parser::FileContent;

pub struct UpetModel {
    module: CModule,
    device: Device
}

impl UpetModel {
    pub fn load<P: AsRef<Path>>(model_path: P, use_gpu: bool) -> Result<Self> {
#[cfg(unix)]
        unsafe {
            // Replace this path with the actual path to the .so file you found in Step 2
            // e.g., ".venv/lib/python3.X/site-packages/metatomic/libmetatomic.so"
            let ext_path = "/home/jay/Projects/rust/qbc/.venv/lib/python3.14/site-packages/metatomic/libmetatomic.so"; 
            
            let flags = libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL;
            let lib = libloading::os::unix::Library::open(Some(ext_path), flags)
                .context("Failed to load metatomic custom ops")?;
            
            // Leak the library so it doesn't get unloaded when `lib` goes out of scope.
            // PyTorch needs these operations registered for the lifetime of the program.
            std::mem::forget(lib);
        }
        let device = if use_gpu && tch::Cuda::is_available() {
            Device::Cuda(0)
        } else {
            Device::Cpu
        };
        
        let module = CModule::load_on_device(model_path, device)?;
        Ok( Self { module, device } )
    }

    pub fn evaluate_frame(&self, frame: &FileContent) {
        todo!()
    }
}
