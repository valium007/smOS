//! Kernel-level exception handling, à la Linux's `__ex_table`.
//!
//! When the kernel executes a privileged instruction that may fault on
//! untrusted/invalid input (e.g. `rdmsr`/`wrmsr` on a non-existent MSR
//! raising `#GP`), we don't want to take down the whole kernel. Instead we
//! record, at *link time*, a table of `(faulting instruction -> fixup code)`
//! pairs. When a CPU exception fires, [`fixup_exception`] looks the faulting
//! `RIP` up in that table; if found, the exception handler rewrites the
//! saved `RIP` to the fixup address and returns, so execution resumes in a
//! small recovery stub instead of panicking.
//!
//! # How the table is built
//!
//! Linux stores each entry as a pair of 32-bit *relative* offsets so the
//! table stays compact and position independent. The COFF/PE equivalent of
//! that relative offset is an image-relative relocation (`@IMGREL`, COFF type
//! `IMAGE_REL_AMD64_ADDR32NB`): a 32-bit RVA measured from `__ImageBase`. At
//! runtime the absolute address of any entry is just `image_base + rva`.
//!
//! Each faulting routine is written in `global_asm!` and emits, into a
//! dedicated `.extab` section, an [`ExtabEntry`] holding the `@IMGREL` of the
//! faulting instruction and of its fixup stub.
//!
//! # Section layout
//!
//! The table is delimited using COFF grouped sections (the same `$`-ordering
//! trick `main.rs` uses for limine requests): the linker concatenates
//! `.extab$a`, `.extab$b`, `.extab$z` into one `.extab` section sorted by the
//! suffix, so the zero-content markers in `$a`/`$z` bracket every entry
//! emitted into `$b`.
//!
//! Note the entries *must* be emitted from `global_asm!`, not inline `asm!`:
//! inline asm treats `$` as operand syntax and mangles the section name, so
//! `.extab$b` would silently land in a separate `.extabb` section outside the
//! markers. `global_asm!` and `#[link_section]` both preserve the `$`.

use core::ptr::addr_of;

/// One exception-table entry: image-relative offsets (RVAs) of the faulting
/// instruction and of the fixup stub to resume at.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExtabEntry {
    /// RVA of the instruction that may fault.
    fault: u32,
    /// RVA of the fixup stub to jump to when it does.
    fixup: u32,
}

// Bracket the entries emitted into `.extab$b`. The linker merges
// `.extab$a`/`.extab$b`/`.extab$z` into `.extab`, ordered by the suffix, so
// these markers sit immediately before/after the entry array. Each is a full
// (non-empty) entry — a zero-sized static is dropped as an empty section,
// breaking the boundary symbols — and the start sentinel is skipped below.
#[used]
#[unsafe(link_section = ".extab$a")]
static EXTAB_START: ExtabEntry = ExtabEntry { fault: 0, fixup: 0 };

#[used]
#[unsafe(link_section = ".extab$z")]
static EXTAB_END: ExtabEntry = ExtabEntry { fault: 0, fixup: 0 };

unsafe extern "C" {
    /// Linker-defined symbol equal to the PE image base. Referencing it goes
    /// through a base relocation, so its address is the *runtime* load base
    /// even when limine relocates the kernel.
    static __ImageBase: u8;
}

/// Runtime base address of the loaded kernel image.
#[inline]
fn image_base() -> u64 {
    addr_of!(__ImageBase) as u64
}

/// The exception table as a slice (excluding the start/end marker sentinels).
fn table() -> &'static [ExtabEntry] {
    // Entries begin right after the start sentinel and run up to the end one.
    let start = addr_of!(EXTAB_START) as usize + size_of::<ExtabEntry>();
    let end = addr_of!(EXTAB_END) as usize;
    let count = (end - start) / size_of::<ExtabEntry>();
    // SAFETY: `start..end` spans the contiguous `.extab` entry array produced
    // by the linker; it lives for the whole image lifetime ('static).
    unsafe { core::slice::from_raw_parts(start as *const ExtabEntry, count) }
}

/// Look `faulting_rip` up in the exception table.
///
/// Returns the absolute address of the fixup stub to resume execution at, or
/// `None` if the fault did not originate from a registered instruction (i.e.
/// it is a genuine bug and the caller should oops/panic).
///
/// This is a linear scan; the table is small. Linux sorts `__ex_table` and
/// binary-searches it — an easy future optimization if the table grows.
pub fn fixup_exception(faulting_rip: u64) -> Option<u64> {
    let base = image_base();
    for entry in table() {
        if base + entry.fault as u64 == faulting_rip {
            return Some(base + entry.fixup as u64);
        }
    }
    None
}

// The faulting routines live in `global_asm!` so the `@IMGREL` table entries
// land in `.extab$b` with the `$` intact (see the module note above).
//
// Both use the Windows x64 calling convention:
//   asm_rdmsr_safe(rcx = msr, rdx = *mut u64 out) -> eax (0 = ok, 1 = #GP)
//   asm_wrmsr_safe(rcx = msr, rdx = u64 value)    -> eax (0 = ok, 1 = #GP)
core::arch::global_asm!(
    ".text",

    ".globl asm_rdmsr_safe",
    "asm_rdmsr_safe:",
    "    mov r8, rdx",            // r8 = out pointer (rdmsr clobbers edx)
    // ecx already holds the MSR index (low half of rcx).
    "2:  rdmsr",                  // edx:eax = MSR value; may #GP -> fixup
    "    mov [r8], eax",          // success: store low ...
    "    mov [r8 + 4], edx",      // ... and high halves
    "    xor eax, eax",           // return 0 (ok)
    "    ret",
    "3:  mov dword ptr [r8], 0",  // fixup: zero the output ...
    "    mov dword ptr [r8 + 4], 0",
    "    mov eax, 1",             // return 1 (faulted)
    "    ret",
    ".pushsection .extab$b, \"dr\"",
    ".p2align 2",
    ".long 2b@IMGREL",           // faulting rdmsr
    ".long 3b@IMGREL",           // its fixup stub
    ".popsection",

    ".globl asm_wrmsr_safe",
    "asm_wrmsr_safe:",
    "    mov eax, edx",           // eax = low 32 of value
    "    mov r8, rdx",
    "    shr r8, 32",
    "    mov edx, r8d",           // edx = high 32 of value
    // ecx already holds the MSR index.
    "4:  wrmsr",                  // may #GP -> fixup
    "    xor eax, eax",           // return 0 (ok)
    "    ret",
    "5:  mov eax, 1",             // return 1 (faulted)
    "    ret",
    ".pushsection .extab$b, \"dr\"",
    ".p2align 2",
    ".long 4b@IMGREL",           // faulting wrmsr
    ".long 5b@IMGREL",           // its fixup stub
    ".popsection",
);

unsafe extern "win64" {
    fn asm_rdmsr_safe(msr: u32, out: *mut u64) -> u32;
    fn asm_wrmsr_safe(msr: u32, value: u64) -> u32;
}

/// Read an MSR, recovering from a `#GP` if the MSR doesn't exist or isn't
/// readable.
///
/// Returns `Ok(value)` on success, `Err(())` if the `rdmsr` faulted.
pub fn rdmsr_safe(msr: u32) -> Result<u64, ()> {
    let mut value: u64 = 0;
    let faulted = unsafe { asm_rdmsr_safe(msr, &mut value) };
    if faulted != 0 { Err(()) } else { Ok(value) }
}

/// Write an MSR, recovering from a `#GP` if the MSR doesn't exist or the
/// value is invalid.
///
/// Returns `Ok(())` on success, `Err(())` if the `wrmsr` faulted.
pub fn wrmsr_safe(msr: u32, value: u64) -> Result<(), ()> {
    let faulted = unsafe { asm_wrmsr_safe(msr, value) };
    if faulted != 0 { Err(()) } else { Ok(()) }
}
