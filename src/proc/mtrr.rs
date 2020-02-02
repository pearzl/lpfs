//! > 5.2.23.  /proc/mtrr
//! > This file refers to the current Memory Type Range Registers (MTRRs) in use with the system. If the system architecture supports MTRRs, then the /proc/mtrr file may look similar to the following:
//! > reg00: base=0x00000000 (   0MB), size= 256MB: write-back, count=1
//! > reg01: base=0xe8000000 (3712MB), size=  32MB: write-combining, count=1
//! > MTRRs are used with the Intel P6 family of processors (Pentium II and higher) and control processor access to memory ranges. When using a video card on a PCI or AGP bus, a properly configured /proc/mtrr file can increase performance more than 150%.
//! > Most of the time, this value is properly configured by default. More information on manually configuring this file can be found locally at the following location:
//! > /usr/share/doc/kernel-doc-<version>/Documentation/mtrr.txt
//! 
//!  -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-mtrr
//! 

//! > MTRR use is replaced on modern x86 hardware with PAT. Direct MTRR use by
//! > drivers on Linux is now completely phased out, device drivers should use
//! > arch_phys_wc_add() in combination with ioremap_wc() to make MTRR effective on
//! > non-PAT systems while a no-op but equally effective on PAT enabled systems.
//! > 
//! > There are two interfaces to /proc/mtrr: one is an ASCII interface
//! > which allows you to read and write. The other is an ioctl()
//! > interface. The ASCII interface is meant for administration. The
//! > ioctl() interface is meant for C programs (i.e. the X server). The
//! > interfaces are described below, with sample commands and C code.
//! 
//! -- https://www.kernel.org/doc/Documentation/x86/mtrr.txt