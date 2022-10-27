use log::{error};

/// Send a message to the team if a critical failure is reached
pub async fn critical_output(e: String) {
	error!("{}", e);
}