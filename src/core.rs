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
    memory_info: MemoryInfo,
    utilization_rates: Utilization,
}

impl GPUStat {
    pub fn new(index: u32, device: &Device) -> Result<GPUStat> {
        Ok(GPUStat {
            index: index,
            uuid: device.uuid()?,
            name: device.name()?,
            memory_info: device.memory_info()?,
            utilization_rates: device.utilization_rates()?,
        })
    }

    pub fn update(&mut self, device: &Device) {
        self.memory_info = device.memory_info().unwrap();
    }
}

#[derive(Debug)]
pub struct GPUStatCollection {
    nvml: NVML,
    gpus: Vec<GPUStat>,
    hostname: String,
    query_time: SystemTime,
}

impl GPUStatCollection {
    pub fn new() -> Result<GPUStatCollection> {
        let nvml = NVML::init()?;

        let device_count = nvml.device_count()?;

        let mut gpus = Vec::new();

        for i in 0..device_count {
            gpus.push(GPUStat::new(i, &nvml.device_by_index(i)?)?);
        }

        let gpu_stat_collection = GPUStatCollection {
            nvml,
            gpus,
            hostname: get_hostname().expect("fail to get hostname"),
            query_time: SystemTime::now(),
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
