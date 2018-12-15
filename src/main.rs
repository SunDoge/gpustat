#[macro_use]
extern crate clap;

use clap::{App, Arg};
use gpustat::GPUStatCollection;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("gpustat")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short("i")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .takes_value(false)
                .help("Print all the information in JSON format"),
        )
        .get_matches();

    // No loop
    let interval = if matches.occurrences_of("interval") == 0 {
        0
    } else {
        value_t!(matches.value_of("interval"), u64).unwrap()
    };

    let mut gpu_stat = GPUStatCollection::new().unwrap();

    if interval > 0 {
        loop {
            print_gpustat(&gpu_stat, matches.is_present("json"));
            thread::sleep(Duration::from_secs(interval));
            gpu_stat.update();
        }
    } else {
        print_gpustat(&gpu_stat, matches.is_present("json"));
    }
}

fn print_gpustat(gpu_stat: &GPUStatCollection, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(gpu_stat).unwrap());
    } else {
        println!("{:#?}", gpu_stat);
    }
}

// fn main() -> nvml_wrapper::error::Result<()> {
//     use nvml_wrapper::NVML;
//     use std::thread;
//     use std::time::Duration;

//     let nvml = NVML::init()?;
//     // Get the first `Device` (GPU) in the system
//     let device0 = nvml.device_by_index(4)?;
//     let device1 = nvml.device_by_index(5)?;

//     println!("{:?}", device0.memory_info()?);
//     println!("{:?}", device1.memory_info()?);
//     thread::sleep(Duration::from_secs(1));
//     println!("{:?}", device0.memory_info()?);
//     println!("{:?}", device1.memory_info()?);
//     thread::sleep(Duration::from_secs(1));
//     println!("{:?}", device0.memory_info()?);
//     println!("{:?}", device1.memory_info()?);
//     thread::sleep(Duration::from_secs(1));
//     println!("{:?}", device0.memory_info()?);
//     println!("{:?}", device1.memory_info()?);
//     thread::sleep(Duration::from_secs(1));

//     Ok(())
// }
