use tch::{CModule, Tensor, Kind, Device};

use crate::parser::FileContent;

pub struct UpetModel {
    module: CModule,
    device: Device
}

impl UpetModel {
    pub fn load() {
        todo!()
    }

    pub fn evaluate_frame(&self, frame: &FileContent) {
        todo!()
    }
}
