# Kettle
An open-source Rust-based Proof-Of-Concept OS project.

# More Information
## What Is Kettle
Kettle is an open-source project consisting of a kernel (Kettle Kernel) and operating system (KettleOS).
## Kettle's Goal
- Create a set of standards for Operating System Development.
- Define standard structures operating systems will follow such as file system layout, executable layout, and dynamic linking.
- Provide a standard by which operating systems (like Linux, or more importantly, Kettle) provide extensions for interacting with other parts of the operating system not added by the kernel, such as providing a desktop API.
- Hardware-based attestation (needs specialized hardware) and other basic security features that would improve the desktop security model.
- Attestation API compatibility with downstream forks (opt-out, based on the presence of security features).
## Non-goals
- Provide a platform in which all executables will work the same on all Operating Systems.
- Provide compatibility for applications that work on other operating systems.
- Make a fully functional operating system on par with Linux. (subject to change)

# Compiling
Be sure to add a `.env` file in the project root and add the line `PLATFORM=virt` where `virt` is your desired platform to compile and emulate. Note that `virt` is the most supported platform.
## Supported Platforms
 - `virt`

# License
Except for all trademarks, logos, and assets, the code in the repository is licensed under [EUPLv1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12) as found in `LICENSE`.
