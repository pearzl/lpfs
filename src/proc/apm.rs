//! /proc/apm
//! 
//! > 
//! > This file provides information about the state of the _Advanced Power Management (APM)_ system and is used by the `apm` command. If a system with no battery is connected to an AC power source, this virtual file would look similar to the following:
//! > 
//! > <pre class="screen">1.16 1.2 0x07 0x01 0xff 0x80 -1% -1 ?</pre>
//! > 
//! > Running the `apm -v` command on such a system results in output similar to the following:
//! > 
//! > <pre class="screen">APM BIOS 1.2 (kernel driver 1.16ac) AC on-line, no system battery</pre>
//! > 
//! > For systems which do not use a battery as a power source, `apm` is able do little more than put the machine in standby mode. The `apm` command is much more useful on laptops. For example, the following output is from the command `cat /proc/apm` on a laptop while plugged into a power outlet:
//! > 
//! > <pre class="screen">1.16 1.2 0x03 0x01 0x03 0x09 100% -1 ?</pre>
//! > 
//! > When the same laptop is unplugged from its power source for a few minutes, the content of the `apm` file changes to something like the following:
//! > 
//! > <pre class="screen">1.16 1.2 0x03 0x00 0x00 0x01 99% 1792 min</pre>
//! > 
//! > The `apm -v` command now yields more useful data, such as the following:
//! > 
//! > <pre class="screen">APM BIOS 1.2 (kernel driver 1.16) AC off-line, battery status high: 99% (1 day, 5:52)</pre>
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-apm
//! >
//! >
//! >
//! > 0) Linux driver version (this will change if format changes)
//! > 1) APM BIOS Version.  Usually 1.0, 1.1 or 1.2.
//! > 2) APM flags from APM Installation Check (0x00):
//! >    bit 0: APM_16_BIT_SUPPORT
//! >    bit 1: APM_32_BIT_SUPPORT
//! >    bit 2: APM_IDLE_SLOWS_CLOCK
//! >    bit 3: APM_BIOS_DISABLED
//! >    bit 4: APM_BIOS_DISENGAGED
//! > 3) AC line status
//! >    0x00: Off-line
//! >    0x01: On-line
//! >    0x02: On backup power (BIOS >= 1.1 only)
//! >    0xff: Unknown
//! > 4) Battery status
//! >    0x00: High
//! >    0x01: Low
//! >    0x02: Critical
//! >    0x03: Charging
//! >    0x04: Selected battery not present (BIOS >= 1.2 only)
//! >    0xff: Unknown
//! > 5) Battery flag
//! >    bit 0: High
//! >    bit 1: Low
//! >    bit 2: Critical
//! >    bit 3: Charging
//! >    bit 7: No system battery
//! >    0xff: Unknown
//! > 6) Remaining battery life (percentage of charge):
//! >    0-100: valid
//! >    -1: Unknown
//! > 7) Remaining battery life (time units):
//! >    Number of remaining minutes or seconds
//! >    -1: Unknown
//! > 8) min = minutes; sec = seconds */
//! >
//! > -- https://github.com/torvalds/linux/blob/86c2f5d653058798703549e1be39a819fcac0d5d/arch/x86/kernel/apm_32.c

define_struct! {
    pub struct Apm {
        driver_version: f32,
        bios_version: f32,
        ac_line_status: u8,
        battery_status: u8,
        battery_flag: u8,
        remain_percent: i8,
        remain_time: u64,
    }
}
