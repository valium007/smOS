This was written for learning purposes to understand how an x86 OS works

- [x] GDT
- [x] IDT (added some interrupts)
- [x] Logging (for qemu)
- [x] x2APIC
- [ ] Physical memory manager(in progress)


# How to run
Make sure you have rust installed and qemu, xorriso and lld-link(llvm linker) in path.
Open cmd/terminal and run 
```
cargo build -Z build-std=core,panic_abort
./run.sh
```
to build the kernel and make the iso to boot in qemu
