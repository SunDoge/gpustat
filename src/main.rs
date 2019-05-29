#[macro_use]
extern crate clap;

use clap::{App, Arg};
use gpustat::GpuMonitor;
use std::thread;
use std::time::Duration;

fn main() {
    let gm = GpuMonitor::new();
}