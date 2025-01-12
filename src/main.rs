// #![feature(alloc_system)]
// extern crate alloc_system;

use chrono::{DateTime, Local, SecondsFormat};
use log::trace;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};
use stderrlog;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "datemon",
    about = "Monitor systemdate and trigger reaction if it jumps too far ahead."
)]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// threshold
    #[structopt(short = "t", long = "threshold", default_value = "86400")]
    threshold: f32,

    /// command to initiate counter measures
    #[structopt(short = "e", long = "exec")]
    exec: Option<String>,

    /// exec_timeout
    #[structopt(long = "exec-timeout", default_value = "300")]
    exec_timeout: f32,

    /// reboot
    #[structopt(short = "r", long = "reboot")]
    reboot: bool,
}

fn execute(cmd: &str) {
    println!("  executing: {}", cmd);
    let mut _child = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("Failed to spawn shell command");
}

fn reboot() {
    // Friendly reboot the system
    println!("Rebooting the system");
    let mut _child = Command::new("/sbin/reboot")
        .spawn()
        .expect("Failed to spawn /sbin/reboot");

    // Forced reboot...
    loop {
        thread::sleep(Duration::from_secs(120));

        println!("Triggering forced reboot");
        let mut _child = Command::new("/sbin/reboot")
            .arg("-f")
            .spawn()
            .expect("Failed to spawn /sbin/reboot -f");
    }
}

fn main() {
    let args = Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbose)
        .init()
        .unwrap();

    println!("Starting datemon: monitoring system for large time/date jumps");

    let threshold = Duration::from_secs_f32(args.threshold);
    let mut prev_time = SystemTime::now();

    loop {
        thread::sleep(Duration::from_secs(1));

        let now_time = SystemTime::now();
        let elapsed = now_time
            .duration_since(prev_time)
            .unwrap_or(Duration::from_secs(0));

        trace!(
            "System time increased by {} seconds this second",
            elapsed.as_secs_f32()
        );

        if elapsed > threshold {
            let prev_date = DateTime::<Local>::from(prev_time);
            let now_date = DateTime::<Local>::from(now_time);
            let elapsed_hours = elapsed.as_secs() / 60 / 60;

            println!(
                "System time jumped from {} to {} (by {} hours)",
                prev_date.to_rfc3339_opts(SecondsFormat::Secs, true),
                now_date.to_rfc3339_opts(SecondsFormat::Secs, true),
                elapsed_hours
            );

            if let Some(cmd) = args.exec.as_ref() {
                execute(cmd);

                // Give the spawed command a chance to remedy the situation
                thread::sleep(Duration::from_secs_f32(args.exec_timeout));

                // Re-check out condition and resume normal operation if we are ok
                let elapsed = prev_time.elapsed().unwrap_or(Duration::from_secs(0));
                if elapsed <= threshold {
                    continue;
                }
            }

            if args.reboot {
                reboot()
            };
        }

        prev_time = now_time;
    }
}
