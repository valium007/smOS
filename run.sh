cp ./target/target/debug/kernel.exe ./iso_root/boot

xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table iso_root -o image.iso

qemu-system-x86_64 -enable-kvm -boot d -cdrom image.iso -m 512 -debugcon stdio
