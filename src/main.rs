#[macro_use] extern crate lazy_static;
extern crate job_scheduler_ng;

use colored::Colorize;
use job_scheduler_ng::{JobScheduler, Job};
use std::time::Duration;
use crate::config::CONFIG;

mod command;
mod config;
mod error;

fn main() {
	// Set the logger
	log4rs::init_file("logging_config.yaml", Default::default()).expect("Logging file is missing.");

	let mut schedule = JobScheduler::new();

    schedule.add(Job::new(CONFIG.server.cron.parse().unwrap(), || {
        restart();
    }));

    loop {
        schedule.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}

fn restart() {
	// Clears the console screen to keep up one message
	print!("\x1B[2J\x1B[1;1H");

	println!("{}", "Restarting the server.".blue());

	command::start_server("file");
}