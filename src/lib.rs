#[macro_use]
extern crate serde_derive;

use chrono::{DateTime, Local};
use hostname::get_hostname;
use nvml_wrapper::error::Result;
use nvml_wrapper::struct_wrappers::device::{MemoryInfo, ProcessInfo, Utilization};
use nvml_wrapper::Device;
use nvml_wrapper::NVML;

#[derive(Debug, Serialize)]
pub struct GPUStat {
    index: u32,
    uuid: String,
    name: String,
    memory: Option<MemoryInfo>,
    utilization: Option<Utilization>,
    processes: Vec<ProcessInfo>,
}

impl GPUStat {
    pub fn new(index: u32, device: &Device) -> GPUStat {
        GPUStat {
            index: index,
            uuid: device.uuid().unwrap(),
            name: device.name().unwrap(),
            memory: device.memory_info().ok(),
            utilization: device.utilization_rates().ok(),
            processes: device.running_compute_processes().unwrap_or(Vec::new()),
        }
    }

    pub fn update(&mut self, device: &Device) {
        if self.memory.is_some() {
            self.memory = device.memory_info().ok();
        }

        if self.utilization.is_some() {
            self.utilization = device.utilization_rates().ok();
        }

        if let Ok(processes) = device.running_compute_processes() {
            self.processes = processes;
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GPUStatCollection {
    #[serde(skip)]
    nvml: NVML,
    gpus: Vec<GPUStat>,
    hostname: String,
    query_time: DateTime<Local>,
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
            query_time: Local::now(),
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
