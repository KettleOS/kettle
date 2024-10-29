use thiserror_no_std::Error;

use crate::driver::error::DriverError;

pub type KernelResult<T> = Result<T, KernelError>;

#[derive(Error, Debug)]
pub enum KernelError {
	#[error("{0}")]
	DriverError(#[from] DriverError),
}
