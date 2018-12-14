#[macro_use]
extern crate serde_derive;

use chrono::{DateTime, Local};
use hostname::get_hostname;
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::error::Result;
use nvml_wrapper::struct_wrappers::device::{MemoryInfo, ProcessInfo, Utilization};
use nvml_wrapper::Device;
use nvml_wrapper::NVML;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_docker_container_id(pid: u32) -> Option<String> {
    let cgroup = format!("/proc/{}/cgroup", pid);
    if Path::new(&cgroup).exists() {
        let f = File::open(&cgroup).unwrap();
        let file = BufReader::new(&f);
        for line in file.lines() {
            let line = line.unwrap();
            if line.contains("docker") {
                let line = line.replace("\n", "");
                let docker_path = line.split(":").last().unwrap();
                let id = docker_path.split("/").last().unwrap();
                return Some(id.to_string());
            }
        }
    }
    None
}

pub fn get_processes(device: &Device) -> Vec<Process> {
    let mut processes = Vec::new();

    if let Ok(running_compute_processes) = device.running_compute_processes() {
        for process_info in running_compute_processes {
            processes.push(Process::from(process_info));
        }
    }

    processes
}

#[derive(Debug, Serialize)]
pub struct Process {
    pid: u32,
    gpu_memory_usage: Option<u64>,
    command: String,
    username: String,
    container_id: Option<String>,
}

impl From<ProcessInfo> for Process {
    fn from(process_info: ProcessInfo) -> Self {
        let process = psutil::process::Process::new(process_info.pid as i32).unwrap();

        Process {
            pid: process_info.pid,
            gpu_memory_usage: match process_info.used_gpu_memory {
                UsedGpuMemory::Used(usage) => Some(usage),
                UsedGpuMemory::Unavailable => None,
            },
            command: process.comm,
            username: users::get_user_by_uid(process.uid)
                .unwrap()
                .name()
                .to_string_lossy()
                .into_owned(),
            container_id: get_docker_container_id(process_info.pid),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GPUStat {
    index: u32,
    uuid: String,
    name: String,
    memory: Option<MemoryInfo>,
    utilization: Option<Utilization>,
    processes: Vec<Process>,
}

impl GPUStat {
    pub fn new(index: u32, device: &Device) -> GPUStat {
        GPUStat {
            index: index,
            uuid: device.uuid().unwrap(),
            name: device.name().unwrap(),
            memory: device.memory_info().ok(),
            utilization: device.utilization_rates().ok(),
            processes: get_processes(device),
        }
    }

    pub fn update(&mut self, device: &Device) {
        if self.memory.is_some() {
            self.memory = device.memory_info().ok();
        }

        if self.utilization.is_some() {
            self.utilization = device.utilization_rates().ok();
        }

        self.processes = get_processes(device);
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

        self.query_time = Local::now();
    }
}
