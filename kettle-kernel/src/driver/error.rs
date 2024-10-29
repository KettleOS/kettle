use thiserror_no_std::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DriverError {
	#[error("driver initialization: {0}")]
	Init(#[from] DriverInitError),
	#[error("duplicate drivers: {name} ({uuid})")]
	Duplicate {
		uuid: Uuid,
		name: &'static str,
	},
	#[error("driver is uninitialized: {name} ({uuid})")]
	Uninitialized {
		uuid: Uuid,
		name: &'static str,
	},
}

#[derive(Error, Debug)]
pub enum DriverInitError {
	#[error("{0}")]
	Other(&'static str),
}
