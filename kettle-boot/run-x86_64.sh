drive=$(cat ./drive)

cd ./target/x86_64-unknown-uefi
sudo qemu-system-x86_64 -drive file=$drive,format=raw -m 1G -pflash ../../../OVMF/OVMF_CODE.fd
