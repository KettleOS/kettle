use std::env;

use dotenv::dotenv;

const PLATFORMS: &'static [&'static str] = &["virt"];

fn main() {
	println!("cargo::rerun-if-changed=.env");

	dotenv().ok();
	let platform = env::var("PLATFORM").expect(format!("Available Platforms: {:}\nThere should be a .env file defining the PLATFORM environment variable", PLATFORMS.join(", ")).as_str());

	// Call linker with linker script
	println!("cargo::rustc-link-arg=-Tkettle-kernel/src/platform/_{platform}/kernel.ld");

	// Compile for platform-specific CPUs
	match platform.as_str() {
		"virt" => println!("cargo::rustc-env=RUSTFLAGS=-Ctarget-cpu=cortex-a72"),
		_ => panic!("Platform {} is not supported", platform),
	}
}
