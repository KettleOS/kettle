cd kettle/target/x86_64-kettle
qemu-system-x86_64 -drive file=boot-uefi-kettle.img,format=raw -m 1G -pflash ../../OVMF/OVMF_CODE.fd
