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

    let interval = if matches.occurrences_of("interval") == 0 {
        0
    } else {
        value_t!(matches.value_of("interval"), u64).unwrap()
    };

    if interval > 0 {
        loop {
            print_gpustat(matches.is_present("json"));
            thread::sleep(Duration::from_secs(interval));
        }
    } else {
        print_gpustat(matches.is_present("json"));
    }
}

fn print_gpustat(json: bool) {
    let gpu_stat = GPUStatCollection::new().unwrap();

    if json {
        println!("{}", serde_json::to_string_pretty(&gpu_stat).unwrap());
    } else {
        println!("{:#?}", gpu_stat);
    }
}
