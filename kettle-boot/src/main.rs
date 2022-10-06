#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use uefi::prelude::*;

#[entry]
fn main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
	uefi_services::init(&mut system_table).unwrap();
	uefi_services::println!("Hello world!");
	
	Status::SUCCESS
}
