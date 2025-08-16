@echo off

copy .\target\target\debug\kernel.exe .\iso_root\boot

xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table iso_root -o image.iso

bochs -f ./bochs/bochsrc -debugger