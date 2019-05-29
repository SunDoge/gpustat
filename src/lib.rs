#[macro_use]
extern crate serde_derive;

use std::sync::Arc;
use nvml_wrapper::{Device, NVML};
use nvml_wrapper::error::Result;


pub struct Memory<'a> {
    device: Arc<Device<'a>>
}

impl<'a> Memory<'a> {
    pub fn new(device: Arc<Device>) -> Memory {
        Memory {
            device
        }
    }
}

pub struct GpuInfo {}

pub struct GpuMonitor {
    nvml: NVML,
    driver_version: String,
    gpus: Vec<GpuInfo>
}

impl GpuMonitor {
    pub fn new() -> GpuMonitor {
        let nvml = NVML::init().unwrap();
        let driver_version = nvml.sys_driver_version().unwrap();


    }
}