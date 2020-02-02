//! > 5.2.25.  /proc/pci
//! > This file contains a full listing of every PCI device on the system. Depending on the number of PCI devices, /proc/pci can be rather long. A sampling of this file from a basic system looks similar to the following:
//! > Bus  0, device 0, function 0: Host bridge: Intel Corporation 440BX/ZX - 82443BX/ZX Host bridge (rev 3). Master Capable. Latency=64. Prefetchable 32 bit memory at 0xe4000000 [0xe7ffffff].
//! > Bus  0, device 1, function 0: PCI bridge: Intel Corporation 440BX/ZX - 82443BX/ZX AGP bridge (rev 3).   Master Capable. Latency=64. Min Gnt=128.
//! > Bus  0, device 4, function 0: ISA bridge: Intel Corporation 82371AB PIIX4 ISA (rev 2).
//! > Bus  0, device 4, function 1: IDE interface: Intel Corporation 82371AB PIIX4 IDE (rev 1). Master Capable. Latency=32. I/O at 0xd800 [0xd80f].
//! > Bus  0, device 4, function 2: USB Controller: Intel Corporation 82371AB PIIX4 USB (rev 1). IRQ 5. Master Capable. Latency=32. I/O at 0xd400 [0xd41f].
//! > Bus  0, device 4, function 3: Bridge: Intel Corporation 82371AB PIIX4 ACPI (rev 2). IRQ 9.
//! > Bus  0, device 9, function 0: Ethernet controller: Lite-On Communications Inc LNE100TX (rev 33). IRQ 5. Master Capable. Latency=32. I/O at 0xd000 [0xd0ff].
//! > Bus  0, device 12, function  0: VGA compatible controller: S3 Inc. ViRGE/DX or /GX (rev 1). IRQ 11. Master Capable. Latency=32. Min Gnt=4.Max Lat=255.
//! > This output shows a list of all PCI devices, sorted in the order of bus, device, and function. Beyond providing the name and version of the device, this list also gives detailed IRQ information so an administrator can quickly look for conflicts.
//! > Note
//! > 
//! > To get a more readable version of this information, type:
//! > lspci -vb
//! > 
//! -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-pci