#[macro_use]
extern crate clap;

pub mod core;

use self::core::GPUStatCollection;
use clap::{App, Arg};
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
        .arg(Arg::with_name("json").long("json").takes_value(false))
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
    let gpu_stats = GPUStatCollection::new().unwrap();

    if json {

    } else {
        println!("{:#?}", gpu_stats);
    }
}
