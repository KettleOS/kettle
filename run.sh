cd target/x86_64-kettle-kernel
qemu-system-x86_64 -drive file=boot-uefi-kettle.img,format=raw -m 1G -pflash ../../OVMF/OVMF_CODE.fd
