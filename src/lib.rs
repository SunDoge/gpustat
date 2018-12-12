#[macro_use]
extern crate serde_derive;

use hostname::get_hostname;
use nvml_wrapper::error::Result;
use nvml_wrapper::struct_wrappers::device::{MemoryInfo, Utilization};
use nvml_wrapper::Device;
use nvml_wrapper::NVML;
use std::time::SystemTime;

#[derive(Debug)]
pub struct GPUStat {
    index: u32,
    uuid: String,
    name: String,
    memory_info: Option<MemoryInfo>,
    utilization_rates: Option<Utilization>,
}

impl GPUStat {
    pub fn new(index: u32, device: &Device) -> GPUStat {
        GPUStat {
            index: index,
            uuid: device.uuid().unwrap(),
            name: device.name().unwrap(),
            memory_info: device.memory_info().ok(),
            utilization_rates: device.utilization_rates().ok(),
        }
    }

    pub fn update(&mut self, device: &Device) {
        if self.memory_info.is_some() {
            self.memory_info = device.memory_info().ok();
        }

        if self.utilization_rates.is_some() {
            self.utilization_rates = device.utilization_rates().ok();
        }
    }
}

#[derive(Debug)]
pub struct GPUStatCollection {
    nvml: NVML,
    gpus: Vec<GPUStat>,
    hostname: String,
    query_time: SystemTime,
    driver_version: String,
}

impl GPUStatCollection {
    pub fn new() -> Result<GPUStatCollection> {
        let nvml = NVML::init()?;

        let device_count = nvml.device_count()?;

        let mut gpus = Vec::new();

        for i in 0..device_count {
            gpus.push(GPUStat::new(i, &nvml.device_by_index(i)?));
        }

        let driver_version = nvml.sys_driver_version()?;

        let gpu_stat_collection = GPUStatCollection {
            nvml,
            gpus,
            hostname: get_hostname().expect("fail to get hostname"),
            query_time: SystemTime::now(),
            driver_version,
        };

        Ok(gpu_stat_collection)
    }

    pub fn update(&mut self) {
        for gpu in &mut self.gpus {
            let device = self.nvml.device_by_index(gpu.index).unwrap();
            gpu.update(&device);
        }
    }
}
