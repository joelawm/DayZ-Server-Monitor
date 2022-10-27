/*-------------
/config.rs

This file is for the connection to the server and tokens.

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use futures::executor::block_on;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use crate::error;

// Global
lazy_static! {
	// Load all the server.toml configs
    pub static ref CONFIG: Config = load_settings();
}

/* Main table for holding each individual  */
#[derive(Debug, Deserialize)]
pub struct Config {
	pub server: ServerConfig,
}

/* Used for server names, etc */
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	pub name: String,
	pub config: String,
	pub night_config: String,
	pub location: String,
	pub executable: String,
	pub port: i16,
	pub cpu: i8,
	pub cron: String,
}

pub fn load_settings() -> Config {
	// Generate file here if not already created
	let mut file = File::open("Server.toml").unwrap_or_else(|e| {
		block_on(error::critical_output(format!("Server - Can't open File! - {}", e)));
		panic!("Failed");
	});

	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap_or_else(|e| {
		block_on(error::critical_output(format!("Server - Oops, cannot read the file! - {}", e)));
		panic!("Failed");
	});

	let decoded: Config = toml::from_str(&mut contents).unwrap();
	return decoded;
}