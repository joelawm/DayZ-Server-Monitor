/*-------------
/parsing/timeline.rs

This file is for parsing the response data from the database to a usable json output.

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use std::process::Command;
use std::io::{self, Write};

/// Parses the timeline to a more usable less raw format.
/// 
/// # Arguments
/// * `r_timeline` - A RawTimelinePost.
/// * `in_offset` - A i32 of the current offset.
pub fn start_server(config: &str) {
	let output = if cfg!(target_os = "windows") {
		//start "DayZ Server" /min "DayZServer_x64.exe" -config=%serverConfig% -port=%serverPort% "-mod=@CF;@Unlimited-Stamina" "-profiles=config" -cpuCount=%serverCPU% -dologs -adminlog -netlog -freezecheck
		Command::new("start 'DayZ Server' /min 'DayZServer_x64.exe'").args([format!("{}{}", "-config=", config)]).output().expect("failed to execute process")
	} else {
		// Filler Command till Linux support is added.
		Command::new("clear").output().expect("failed to execute process")
	};

	println!("status: {}", output.status);
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();
}

pub fn kill_server() {
	//taskkill /im DayZServer_x64.exe /F
}