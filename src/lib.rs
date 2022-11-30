mod cpus_info;

use crate::cpus_info::CpusInfo;
use clap::Parser;
use core::time;
use std::thread;


/// Dump Cpu usage every T (default 300) milliseconds
/// 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Millisecond
    #[clap(short, long)]
    time: Option<u64>,
}



pub fn main_impl() {
    let mut cpu_info = CpusInfo::new();
    let args = Args::parse();
    let time = args.time.unwrap_or(300);
    let ten_millis = time::Duration::from_millis(time);
    cpu_info.print_header();
    loop {
        thread::sleep(ten_millis);
        cpu_info.update_and_print();
        println!();
    }
}

#[cfg(test)]
mod main_test {
    use super::*;

    #[test]
    fn stub_main() {
        let mut cpu_info = CpusInfo::new();
        let args = Args::parse();
        let time = args.time.unwrap_or(300);
        let ten_millis = time::Duration::from_millis(time);
        cpu_info.print_header();
        for _ in 0..20 {
            thread::sleep(ten_millis);
            cpu_info.update_and_print();
            println!();
        }
    }
}
